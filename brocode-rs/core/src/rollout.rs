use crate::config::Config;
pub use brocode_rollout::ARCHIVED_SESSIONS_SUBDIR;
pub use brocode_rollout::INTERACTIVE_SESSION_SOURCES;
pub use brocode_rollout::RolloutRecorder;
pub use brocode_rollout::RolloutRecorderParams;
pub use brocode_rollout::SESSIONS_SUBDIR;
pub use brocode_rollout::SessionMeta;
pub use brocode_rollout::append_thread_name;
pub use brocode_rollout::find_archived_thread_path_by_id_str;
#[deprecated(note = "use find_thread_path_by_id_str")]
pub use brocode_rollout::find_conversation_path_by_id_str;
pub use brocode_rollout::find_thread_name_by_id;
pub use brocode_rollout::find_thread_path_by_id_str;
pub use brocode_rollout::find_thread_path_by_name_str;
pub use brocode_rollout::rollout_date_parts;

impl brocode_rollout::RolloutConfigView for Config {
    fn brocode_home(&self) -> &std::path::Path {
        self.brocode_home.as_path()
    }

    fn sqlite_home(&self) -> &std::path::Path {
        self.sqlite_home.as_path()
    }

    fn cwd(&self) -> &std::path::Path {
        self.cwd.as_path()
    }

    fn model_provider_id(&self) -> &str {
        self.model_provider_id.as_str()
    }

    fn generate_memories(&self) -> bool {
        self.memories.generate_memories
    }
}

pub mod list {
    pub use brocode_rollout::list::*;
}

pub(crate) mod metadata {
    pub(crate) use brocode_rollout::metadata::builder_from_items;
}

pub mod policy {
    pub use brocode_rollout::policy::*;
}

pub mod recorder {
    pub use brocode_rollout::recorder::*;
}

pub mod session_index {
    pub use brocode_rollout::session_index::*;
}

pub(crate) use crate::session_rollout_init_error::map_session_init_error;

pub(crate) mod truncation {
    pub(crate) use crate::thread_rollout_truncation::*;
}
