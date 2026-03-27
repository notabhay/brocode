use crate::agent::AgentStatus;
use crate::brocode::Brocode;
use crate::brocode::SteerInputError;
use crate::config::ConstraintResult;
use crate::error::BrocodeErr;
use crate::error::Result as BrocodeResult;
use crate::file_watcher::WatchRegistration;
use crate::protocol::Event;
use crate::protocol::Op;
use crate::protocol::Submission;
use brocode_features::Feature;
use brocode_protocol::config_types::ApprovalsReviewer;
use brocode_protocol::config_types::Personality;
use brocode_protocol::config_types::ServiceTier;
use brocode_protocol::models::ContentItem;
use brocode_protocol::models::ResponseInputItem;
use brocode_protocol::models::ResponseItem;
use brocode_protocol::openai_models::ReasoningEffort;
use brocode_protocol::protocol::AskForApproval;
use brocode_protocol::protocol::SandboxPolicy;
use brocode_protocol::protocol::SessionSource;
use brocode_protocol::protocol::TokenUsage;
use brocode_protocol::protocol::W3cTraceContext;
use brocode_protocol::user_input::UserInput;
use std::path::PathBuf;
use tokio::sync::Mutex;
use tokio::sync::watch;

use crate::state_db::StateDbHandle;

#[derive(Clone, Debug)]
pub struct ThreadConfigSnapshot {
    pub model: String,
    pub model_provider_id: String,
    pub service_tier: Option<ServiceTier>,
    pub approval_policy: AskForApproval,
    pub approvals_reviewer: ApprovalsReviewer,
    pub sandbox_policy: SandboxPolicy,
    pub cwd: PathBuf,
    pub ephemeral: bool,
    pub reasoning_effort: Option<ReasoningEffort>,
    pub personality: Option<Personality>,
    pub session_source: SessionSource,
}

pub struct BrocodeThread {
    pub(crate) brocode: Brocode,
    rollout_path: Option<PathBuf>,
    out_of_band_elicitation_count: Mutex<u64>,
    _watch_registration: WatchRegistration,
}

/// Conduit for the bidirectional stream of messages that compose a thread
/// (formerly called a conversation) in Brocode.
impl BrocodeThread {
    pub(crate) fn new(
        brocode: Brocode,
        rollout_path: Option<PathBuf>,
        watch_registration: WatchRegistration,
    ) -> Self {
        Self {
            brocode,
            rollout_path,
            out_of_band_elicitation_count: Mutex::new(0),
            _watch_registration: watch_registration,
        }
    }

    pub async fn submit(&self, op: Op) -> BrocodeResult<String> {
        self.brocode.submit(op).await
    }

    pub async fn shutdown_and_wait(&self) -> BrocodeResult<()> {
        self.brocode.shutdown_and_wait().await
    }

    pub async fn submit_with_trace(
        &self,
        op: Op,
        trace: Option<W3cTraceContext>,
    ) -> BrocodeResult<String> {
        self.brocode.submit_with_trace(op, trace).await
    }

    pub async fn steer_input(
        &self,
        input: Vec<UserInput>,
        expected_turn_id: Option<&str>,
    ) -> Result<String, SteerInputError> {
        self.brocode.steer_input(input, expected_turn_id).await
    }

    pub async fn set_app_server_client_name(
        &self,
        app_server_client_name: Option<String>,
    ) -> ConstraintResult<()> {
        self.brocode
            .set_app_server_client_name(app_server_client_name)
            .await
    }

    /// Use sparingly: this is intended to be removed soon.
    pub async fn submit_with_id(&self, sub: Submission) -> BrocodeResult<()> {
        self.brocode.submit_with_id(sub).await
    }

    pub async fn next_event(&self) -> BrocodeResult<Event> {
        self.brocode.next_event().await
    }

    pub async fn agent_status(&self) -> AgentStatus {
        self.brocode.agent_status().await
    }

    pub(crate) fn subscribe_status(&self) -> watch::Receiver<AgentStatus> {
        self.brocode.agent_status.clone()
    }

    pub(crate) async fn total_token_usage(&self) -> Option<TokenUsage> {
        self.brocode.session.total_token_usage().await
    }

    /// Records a user-role session-prefix message without creating a new user turn boundary.
    pub(crate) async fn inject_user_message_without_turn(&self, message: String) {
        let message = ResponseItem::Message {
            id: None,
            role: "user".to_string(),
            content: vec![ContentItem::InputText { text: message }],
            end_turn: None,
            phase: None,
        };
        let pending_item = match pending_message_input_item(&message) {
            Ok(pending_item) => pending_item,
            Err(err) => {
                debug_assert!(false, "session-prefix message append should succeed: {err}");
                return;
            }
        };
        if self
            .brocode
            .session
            .inject_response_items(vec![pending_item])
            .await
            .is_err()
        {
            let turn_context = self.brocode.session.new_default_turn().await;
            self.brocode
                .session
                .record_conversation_items(turn_context.as_ref(), &[message])
                .await;
        }
    }

    /// Append a prebuilt message to the thread history without treating it as a user turn.
    ///
    /// If the thread already has an active turn, the message is queued as pending input for that
    /// turn. Otherwise it is queued at session scope and a regular turn is started so the agent
    /// can consume that pending input through the normal turn pipeline.
    #[cfg(test)]
    pub(crate) async fn append_message(&self, message: ResponseItem) -> BrocodeResult<String> {
        let submission_id = uuid::Uuid::new_v4().to_string();
        let pending_item = pending_message_input_item(&message)?;
        if let Err(items) = self
            .brocode
            .session
            .inject_response_items(vec![pending_item])
            .await
        {
            self.brocode
                .session
                .queue_response_items_for_next_turn(items)
                .await;
            self.brocode
                .session
                .ensure_task_for_queued_response_items()
                .await;
        }

        Ok(submission_id)
    }

    pub fn rollout_path(&self) -> Option<PathBuf> {
        self.rollout_path.clone()
    }

    pub fn state_db(&self) -> Option<StateDbHandle> {
        self.brocode.state_db()
    }

    pub async fn config_snapshot(&self) -> ThreadConfigSnapshot {
        self.brocode.thread_config_snapshot().await
    }

    pub fn enabled(&self, feature: Feature) -> bool {
        self.brocode.enabled(feature)
    }

    pub async fn increment_out_of_band_elicitation_count(&self) -> BrocodeResult<u64> {
        let mut guard = self.out_of_band_elicitation_count.lock().await;
        let was_zero = *guard == 0;
        *guard = guard.checked_add(1).ok_or_else(|| {
            BrocodeErr::Fatal("out-of-band elicitation count overflowed".to_string())
        })?;

        if was_zero {
            self.brocode
                .session
                .set_out_of_band_elicitation_pause_state(/*paused*/ true);
        }

        Ok(*guard)
    }

    pub async fn decrement_out_of_band_elicitation_count(&self) -> BrocodeResult<u64> {
        let mut guard = self.out_of_band_elicitation_count.lock().await;
        if *guard == 0 {
            return Err(BrocodeErr::InvalidRequest(
                "out-of-band elicitation count is already zero".to_string(),
            ));
        }

        *guard -= 1;
        let now_zero = *guard == 0;
        if now_zero {
            self.brocode
                .session
                .set_out_of_band_elicitation_pause_state(/*paused*/ false);
        }

        Ok(*guard)
    }
}

fn pending_message_input_item(message: &ResponseItem) -> BrocodeResult<ResponseInputItem> {
    match message {
        ResponseItem::Message { role, content, .. } => Ok(ResponseInputItem::Message {
            role: role.clone(),
            content: content.clone(),
        }),
        _ => Err(BrocodeErr::InvalidRequest(
            "append_message only supports ResponseItem::Message".to_string(),
        )),
    }
}
