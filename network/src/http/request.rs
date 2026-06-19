// network lib src/router_v2/request.rs -- Http Api 请求体
// 2026-03-11 06:51:10

use std::collections::HashMap;

/// # Http Api 请求体

#[derive(Debug, Clone)]
pub struct ApiRequest {
    pub request_id: String,                    // 请求id
    pub api_key: String,                       // api秘钥
    pub path: String,                          // 路径
    pub headers: HashMap<String, Vec<String>>, // 头部
    pub body: Vec<u8>,                         // 主体
}



pub type Headers = HashMap<String, Vec<String>>;

pub struct Body(pub Vec<u8>);