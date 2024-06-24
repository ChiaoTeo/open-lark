use async_recursion::async_recursion;
use lazy_static::lazy_static;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::core::{
    api_req::ApiRequest,
    api_resp::{BaseResponse, RawResponse},
    cache::QuickCache,
    config::Config,
    constants::{AccessTokenType, APPLY_APP_TICKET_PATH, APP_TICKET_KEY_PREFIX},
    http::Transport,
    SDKResult,
};

lazy_static! {
    pub static ref APP_TICKET_MANAGER: AppTicketManager = AppTicketManager::new();
}

pub struct AppTicketManager {
    pub cache: QuickCache<String>,
}

impl Default for AppTicketManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AppTicketManager {
    pub fn new() -> Self {
        Self {
            cache: QuickCache::new(),
        }
    }

    pub fn set(&mut self, app_id: &str, value: &str, expire_time: i32) {
        let key = app_ticket_key(app_id);
        self.cache.set(&key, value.to_string(), expire_time);
    }

    pub async fn get(&self, config: &Config) -> Option<String> {
        let key = app_ticket_key(&config.app_id);
        match self.cache.get(&key) {
            None => None,
            Some(ticket) => {
                if ticket.is_empty() {
                    apply_app_ticket(config).await.ok();
                }

                Some(ticket)
            }
        }
    }
}

fn app_ticket_key(app_id: &str) -> String {
    format!("{}-{}", APP_TICKET_KEY_PREFIX, app_id)
}

#[async_recursion]
pub async fn apply_app_ticket(config: &Config) -> SDKResult<()> {
    let _resp: BaseResponse<RawResponse> = Transport::request(
        ApiRequest {
            http_method: Method::POST,
            api_path: APPLY_APP_TICKET_PATH.to_string(),
            supported_access_token_types: vec![AccessTokenType::App],
            ..Default::default()
        },
        config,
        None,
    )
    .await?;

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct ResendAppTicketReq {
    app_id: String,
    app_secret: String,
}

// #[derive(Serialize, Deserialize)]
// struct ResendAppTicketResp {
//     #[serde(skip)]
//     api_resp:
//     #[serde(flatten)]
//     code_error: ErrorResponse,
// }
