// repo_adapter/src/wallet/point.rs
// ✅ ADAPTER - WALLET 积分账户初始化适配器
// 2026/8/20 Created.

////////

use async_trait::async_trait;
use cola_data::wallet::command::point::WalletPointInitCommand;
use cola_data::wallet::port::point::WalletPointInitResult;
use port::wallet::point::WalletPointPort;
use repository::cola_finance::pg::point_repo::WalletPointRepo;
use tracing::{error, info};

////////

/// # [ADAPTER] - 钱包积分账户初始化适配器
#[derive(Debug, Default, Clone)]
pub struct WalletPointInitAdapter;

////////

#[async_trait]
impl WalletPointPort for WalletPointInitAdapter {
    /// # 1. [ADAPTER] - 初始化积分账户
    /// * `desc`: `将钱包 Port 请求转发至积分仓储事务`
    async fn init_point_account(
        &self,
        cmd: WalletPointInitCommand,
    ) -> anyhow::Result<WalletPointInitResult> {
        let user_id = cmd.user_id;
        match WalletPointRepo::init_point_account(cmd).await {
            Ok(record) => {
                info!(
                    "[🗣️ ADAPTER] - ✅️ POINT账户初始化成功: user_id={}, account_id={}, balance={}, is_new={}",
                    record.user_id, record.account_id, record.balance, record.is_new_account
                );
                Ok(WalletPointInitResult {
                    account_id: record.account_id,
                    user_id: record.user_id,
                    balance: record.balance,
                    transaction_id: record.transaction_id,
                    is_new_account: record.is_new_account,
                })
            }
            Err(error) => {
                error!(
                    "[🤐 ADAPTER] - ❌️ POINT账户初始化失败: user_id={}, error={}",
                    user_id, error
                );
                Err(error)
            }
        }
    }
}

//////// END
