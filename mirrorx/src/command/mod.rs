mod add_domain;
mod delete_domain;
mod generate_random_password;
mod get_current_domain;
mod get_domains;
mod get_language;
mod init_config;
mod init_signaling;
mod set_current_domain_device_password;
mod set_domain_remarks;
mod set_language;
mod signaling_key_exchange;
mod signaling_reply_visit_request;
mod signaling_visit_request;
mod switch_primary_domain;

use mirrorx_core::api::{config::entity::domain::Domain, signaling::SignalingProvider};
use tauri::async_runtime::Mutex;

pub use add_domain::*;
pub use delete_domain::*;
pub use generate_random_password::*;
pub use get_current_domain::*;
pub use get_domains::*;
pub use get_language::*;
pub use init_config::*;
pub use init_signaling::*;
pub use set_current_domain_device_password::*;
pub use set_domain_remarks::*;
pub use set_language::*;
pub use signaling_key_exchange::*;
pub use signaling_reply_visit_request::*;
pub use signaling_visit_request::*;
pub use switch_primary_domain::*;

pub struct UIState {
    domain: Mutex<Option<Domain>>,
    signaling_client: Mutex<Option<SignalingProvider>>,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            domain: Mutex::new(None),
            signaling_client: Mutex::new(None),
        }
    }
}
