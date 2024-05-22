use std::collections::HashMap;

use crate::core::constants::AccessTokenType;

#[derive(Debug, Clone, Default)]
pub struct ApiReq {
    pub(crate) http_method: String,
    pub api_path: String,
    pub body: Vec<u8>,
    pub query_params: HashMap<String, String>,
    pub path_params: HashMap<String, Vec<String>>,
    pub(crate) supported_access_token_types: Vec<AccessTokenType>,
    pub file: Vec<u8>,
}
