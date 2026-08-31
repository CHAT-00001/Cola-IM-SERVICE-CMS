// port/src/wallet/music.rs -- 端口 - WALLET - mod
// 2026/8/20 Created.

////////

use point::WalletPointPort;
use std::sync::Arc;

////////

pub mod point;

////////

/// # [PORTS] - 钱包端口聚合
/// * `desc`: `聚合积分、账户、流水等钱包领域端口`
#[derive(Clone)]
pub struct WalletPort {
    pub point: Arc<dyn WalletPointPort + Send + Sync + 'static>, // 积分账户
}

//////// END
