//! Implements the collaboration tool surface for spawning and managing sub-agents.
//!
//! This handler translates model tool calls into `AgentControl` operations and keeps spawned
//! agents aligned with the live turn that created them. Sub-agents start from the turn's effective
//! config, inherit runtime-only state such as provider, approval policy, sandbox, and cwd, and
//! then optionally layer role-specific config on top.

use crate::agent::AgentStatus;
use crate::agent::exceeds_thread_spawn_depth_limit;
use crate::brocode::Session;
use crate::brocode::TurnContext;
use crate::function_tool::FunctionCallError;
use crate::tools::context::ToolInvocation;
use crate::tools::context::ToolOutput;
use crate::tools::context::ToolPayload;
pub(crate) use crate::tools::handlers::multi_agents_common::*;
use crate::tools::handlers::parse_arguments;
use crate::tools::registry::ToolHandler;
use crate::tools::registry::ToolKind;
use async_trait::async_trait;
use brocode_protocol::ThreadId;
use brocode_protocol::models::ResponseInputItem;
use brocode_protocol::openai_models::ReasoningEffort;
use brocode_protocol::protocol::CollabAgentInteractionBeginEvent;
use brocode_protocol::protocol::CollabAgentInteractionEndEvent;
use brocode_protocol::protocol::CollabAgentRef;
use brocode_protocol::protocol::CollabAgentSpawnBeginEvent;
use brocode_protocol::protocol::CollabAgentSpawnEndEvent;
use brocode_protocol::protocol::CollabCloseBeginEvent;
use brocode_protocol::protocol::CollabCloseEndEvent;
use brocode_protocol::protocol::CollabResumeBeginEvent;
use brocode_protocol::protocol::CollabResumeEndEvent;
use brocode_protocol::protocol::CollabWaitingBeginEvent;
use brocode_protocol::protocol::CollabWaitingEndEvent;
use brocode_protocol::user_input::UserInput;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value as JsonValue;

pub(crate) fn parse_agent_id_target(target: &str) -> Result<ThreadId, FunctionCallError> {
    ThreadId::from_string(target).map_err(|err| {
        FunctionCallError::RespondToModel(format!("invalid agent id {target}: {err:?}"))
    })
}

pub(crate) fn parse_agent_id_targets(
    targets: Vec<String>,
) -> Result<Vec<ThreadId>, FunctionCallError> {
    if targets.is_empty() {
        return Err(FunctionCallError::RespondToModel(
            "agent ids must be non-empty".to_string(),
        ));
    }

    targets
        .into_iter()
        .map(|target| parse_agent_id_target(&target))
        .collect()
}

pub(crate) use close_agent::Handler as CloseAgentHandler;
pub(crate) use resume_agent::Handler as ResumeAgentHandler;
pub(crate) use send_input::Handler as SendInputHandler;
pub(crate) use spawn::Handler as SpawnAgentHandler;
pub(crate) use wait::Handler as WaitAgentHandler;

pub mod close_agent;
mod resume_agent;
mod send_input;
mod spawn;
pub(crate) mod wait;

#[cfg(test)]
#[path = "multi_agents_tests.rs"]
mod tests;
