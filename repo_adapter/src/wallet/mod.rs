// repo_adapter/src/wallet/mod.rs
// ✅ ADAPTER - WALLET 钱包适配器聚合
// 2026/8/20 Updated.

////////

use port::wallet::WalletPort;
use std::sync::Arc;

////////

pub mod point;

////////

/// # [BUILD] - 构建 WALLET Port
/// * `desc`: `统一装配钱包领域的 Port / Adapter`
pub fn build_wallet_port() -> WalletPort {
    WalletPort {
        point: Arc::new(point::WalletPointInitAdapter),
    }
}

//////// END
