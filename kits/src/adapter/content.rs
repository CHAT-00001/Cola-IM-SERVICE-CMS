// kits/src/adapter/content.rs  -- 工具包 - 适配器  - 内容风控
// 2026/5/21 02:05 by wx: cestbon10080
// * 
// * --------

////////

use async_trait::async_trait;
use crate::port::content::RiskControlPort;
////////



pub struct LocalRiskAdapter;

#[async_trait]
impl RiskControlPort for LocalRiskAdapter {
    async fn check_risk(&self, content: &str) -> Result<i16, String> {
        // 专心写本地的正则匹配、布隆过滤器过滤逻辑，不用管超时和降级
        Ok(5)
    }
}

// cola_video/src/video/adapter/risk/third_party.rs
pub struct ThirdPartyRiskAdapter {
    // client: reqwest::Client
}

#[async_trait]
impl RiskControlPort for ThirdPartyRiskAdapter {
    async fn check_risk(&self, content: &str) -> Result<i16, String> {
        // 专心发网络请求给腾讯云或网易易盾，不用管超时和降级
        Ok(5)
    }
}

// * --------
//////// END