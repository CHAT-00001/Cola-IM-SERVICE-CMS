// kits/src/app/content  -- 工具 - PORT - 内容
// 2026/5/21 01:43 by wx: cestbon10080
// * 
// * --------

////////

use async_trait::async_trait;
use std::time::Duration;
use log::log;

////////


/// # [SERVICE] - 风险控制服务端口
/// *
#[async_trait]
pub trait RiskControlPort: Send + Sync + 'static {
    async fn check_risk(&self, content: &str) -> Result<i16, String>;
}



/// # [PLUG] - 风控安全插头
/// * 作用：包裹任意一个风控插座，提供 500ms 超时和降级返回 5 的兜底能力
pub async fn plug_risk_with_fallback(
    risk_port: &dyn RiskControlPort,
    content: &str,
) -> i16 {
    // 💡 把底层的异步调用塞进 timeout 里
    match tokio::time::timeout(Duration::from_millis(500), risk_port.check_risk(content)).await {
        // 情况 1：在时间内成功返回结果
        Ok(Ok(level)) => level,

        // 情况 2：接口报错了 -> 降级返回 5
        Ok(Err(e)) => {
            log::warn!("PLUG - 风控执行报错，触发插头降级(返回5): {}", e);
            5
        }

        // 情况 3：500ms 超时了 -> 降级返回 5
        Err(_) => {
            log::warn!("PLUG - 风控调用超时，触发插头降级(返回5)");
            5
        }
    }
}


// * --------
//////// END