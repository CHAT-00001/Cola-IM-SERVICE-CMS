// port/src/wallet/point.rs
// ✅ PORT - WALLET 积分账户初始化
// 2026/8/20 Created.

////////

use cola_data::wallet::command::point::WalletPointInitCommand;
use cola_data::wallet::port::point::WalletPointInitResult;

////////

/// # [PORT] - 初始化用户积分账户
/// * `desc`: `创建 POINT 账户，并在有赠送积分时生成首笔积分交易`
#[async_trait::async_trait]
pub trait WalletPointPort: Send + Sync {
    /// # 1. [PORT] - 初始化积分账户
    /// * `desc`: `幂等创建用户 POINT 账户；积分最小单位为1`
    async fn init_point_account(
        &self,
        cmd: WalletPointInitCommand,
    ) -> anyhow::Result<WalletPointInitResult>;
}

//////// END
