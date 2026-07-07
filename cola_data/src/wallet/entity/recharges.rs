// /recharges.rs  --
// 2026/6/26 01:48

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, types::JsonValue};

////////

/// # [ENTITY] - 充值记录表
/// * `table_name`: `wallet_recharges`
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct WalletRecharge {
    pub id: i64,                           // BIGSERIAL 主键
    pub tx_no: String,                     // 订单交易流水号（唯一）
    pub user_id: i64,                      // 用户ID
    pub wallet_id: i64,                    // 钱包ID
    pub amount: i64,                       // 充值金额（分）
    pub bonus: i64,                        // 赠送金额
    pub actual_amount: i64,                // 实际到账金额
    pub status: i16,                       // 状态：0-待支付 1-支付中 2-成功 3-失败 4-已取消
    pub channel: String,                   // 支付渠道：WECHAT, ALIPAY, BANK, APPLE, GOOGLE
    pub channel_order_no: Option<String>,  // 渠道订单号
    pub channel_tx_no: Option<String>,     // 渠道交易号
    pub pay_url: Option<String>,           // 支付链接
    pub qr_code: Option<String>,           // 二维码图片（base64）
    pub expired_at: Option<DateTime<Utc>>, // 订单过期时间
    pub paid_at: Option<DateTime<Utc>>,    // 支付完成时间
    pub fail_reason: Option<String>,       // 失败原因
    pub extra: JsonValue,                  // 扩展信息（JSONB）
    pub created_at: DateTime<Utc>,         // 创建时间
    pub updated_at: DateTime<Utc>,         // 更新时间
}
