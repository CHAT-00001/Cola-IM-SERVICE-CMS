// entity/wallet_account.rs  -- entity  钱包账户
// 2026/6/26 01:40

////////

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

////////

/// # [ENTITY] - 钱包账户表
/// * `table name`: `wallet_accounts`
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct WalletAccount {
    pub id: i64,                                 // 账户ID
    pub user_id: i64,                            // 用户ID（关联 user.id）
    pub currency_id: i16,                        // 货币ID（关联币种表）
    pub balance: Decimal,                        // 余额（单位：分/个）
    pub frozen_balance: Decimal,                 // 冻结余额（提现中/纠纷中）
    pub total_income: Decimal,                   // 累计收入
    pub total_expense: Decimal,                  // 累计支出
    pub pay_password: Option<String>,            // 支付密码（bcrypt加密）
    pub pay_password_set: bool,                  // 是否设置支付密码
    pub status: i16,                             // 状态：0-冻结 1-正常 2-注销
    pub freeze_reason: Option<String>,           // 冻结原因
    pub daily_limit: Option<Decimal>,            // 单日限额（0=不限）
    pub single_limit: Option<Decimal>,           // 单笔限额（0=不限）
    pub version: i64,                            // 乐观锁版本号
    pub last_transfer_at: Option<DateTime<Utc>>, // 最近一次转账时间
    pub created_at: DateTime<Utc>,               // 创建时间
    pub updated_at: DateTime<Utc>,               // 更新时间
}

//////// END
