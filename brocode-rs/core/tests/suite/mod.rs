// Aggregates all former standalone integration tests as modules.
use std::ffi::OsString;
use std::path::Path;

use brocode_apply_patch::BROCODE_CORE_APPLY_PATCH_ARG1;
use brocode_arg0::Arg0PathEntryGuard;
use brocode_arg0::arg0_dispatch;
use brocode_sandboxing::landlock::BROCODE_LINUX_SANDBOX_ARG0;
use ctor::ctor;
use tempfile::TempDir;

struct TestBrocodeAliasesGuard {
    _brocode_home: TempDir,
    _arg0: Arg0PathEntryGuard,
    _previous_brocode_home: Option<OsString>,
}

const BROCODE_HOME_ENV_VAR: &str = "BROCODE_HOME";

// This code runs before any other tests are run.
// It allows the test binary to behave like brocode and dispatch to apply_patch and brocode-linux-sandbox
// based on the arg0.
// NOTE: this doesn't work on ARM
#[ctor]
pub static BROCODE_ALIASES_TEMP_DIR: Option<TestBrocodeAliasesGuard> = {
    let mut args = std::env::args_os();
    let argv0 = args.next().unwrap_or_default();
    let exe_name = Path::new(&argv0)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");
    let argv1 = args.next().unwrap_or_default();
    // Helper re-execs inherit this ctor too, but they may run inside a sandbox
    // where creating another BROCODE_HOME tempdir under /tmp is not allowed.
    if exe_name == BROCODE_LINUX_SANDBOX_ARG0 || argv1 == BROCODE_CORE_APPLY_PATCH_ARG1 {
        return None;
    }

    #[allow(clippy::unwrap_used)]
    let brocode_home = tempfile::Builder::new()
        .prefix("brocode-core-tests")
        .tempdir()
        .unwrap();
    let previous_brocode_home = std::env::var_os(BROCODE_HOME_ENV_VAR);
    // arg0_dispatch() creates helper links under BROCODE_HOME/tmp. Point it at a
    // test-owned temp dir so startup never mutates the developer's real ~/.brocode.
    //
    // Safety: #[ctor] runs before tests start, so no test threads exist yet.
    unsafe {
        std::env::set_var(BROCODE_HOME_ENV_VAR, brocode_home.path());
    }

    #[allow(clippy::unwrap_used)]
    let arg0 = arg0_dispatch().unwrap();
    // Restore the process environment immediately so later tests observe the
    // same BROCODE_HOME state they started with.
    match previous_brocode_home.as_ref() {
        Some(value) => unsafe {
            std::env::set_var(BROCODE_HOME_ENV_VAR, value);
        },
        None => unsafe {
            std::env::remove_var(BROCODE_HOME_ENV_VAR);
        },
    }

    Some(TestBrocodeAliasesGuard {
        _brocode_home: brocode_home,
        _arg0: arg0,
        _previous_brocode_home: previous_brocode_home,
    })
};

#[cfg(not(target_os = "windows"))]
mod abort_tasks;
mod agent_jobs;
mod agent_websocket;
mod apply_patch_cli;
#[cfg(not(target_os = "windows"))]
mod approvals;
mod auth_refresh;
mod brocode_delegate;
mod cli_stream;
mod client;
mod client_websockets;
mod code_mode;
mod collaboration_instructions;
mod compact;
mod compact_remote;
mod compact_resume_fork;
mod deprecation_notice;
mod exec;
mod exec_policy;
mod fork_thread;
mod hierarchical_agents;
#[cfg(not(target_os = "windows"))]
mod hooks;
mod image_rollout;
mod items;
mod js_repl;
mod json_result;
mod live_cli;
mod live_reload;
mod memories;
mod model_info_overrides;
mod model_overrides;
mod model_switching;
mod model_visible_layout;
mod models_cache_ttl;
mod models_etag_responses;
mod otel;
mod pending_input;
mod permissions_messages;
mod personality;
mod personality_migration;
mod plugins;
mod prompt_caching;
mod quota_exceeded;
mod realtime_conversation;
mod remote_env;
mod remote_models;
mod request_compression;
#[cfg(not(target_os = "windows"))]
mod request_permissions;
#[cfg(not(target_os = "windows"))]
mod request_permissions_tool;
mod request_user_input;
mod resume;
mod resume_warning;
mod review;
mod rmcp_client;
mod rollout_list_find;
mod safety_check_downgrade;
mod search_tool;
mod seatbelt;
mod shell_command;
mod shell_serialization;
mod shell_snapshot;
mod skill_approval;
mod skills;
mod spawn_agent_description;
mod sqlite_state;
mod stream_error_allows_next_turn;
mod stream_no_completed;
mod subagent_notifications;
mod text_encoding_fix;
mod tool_harness;
mod tool_parallelism;
mod tool_suggest;
mod tools;
mod truncation;
mod turn_state;
mod undo;
mod unified_exec;
mod unstable_features_warning;
mod user_notification;
mod user_shell_cmd;
mod view_image;
mod web_search;
mod websocket_fallback;
