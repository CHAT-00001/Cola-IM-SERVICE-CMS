// repository/src/cola_finance/pg/point_repo.rs
// ✅ REPOSITORY - WALLET 积分账户初始化
// 2026/8/20 Created.

////////

use crate::pg_pool;
use anyhow::{Context, Result, anyhow};
use cola_data::wallet::command::point::WalletPointInitCommand;
use rust_decimal::{Decimal, prelude::ToPrimitive};
use sqlx::{PgPool, Row};

////////

/// # [REPOSITORY] - 积分账户初始化结果
/// * `desc`: `返回账户ID、余额和首笔交易ID`
#[derive(Debug, Clone)]
pub struct PointAccountInitRecord {
    pub account_id: i64,
    pub user_id: i64,
    pub balance: i64,
    pub transaction_id: Option<i64>,
    pub is_new_account: bool,
}

////////

/// # [REPOSITORY] - 钱包积分账户仓储
pub struct WalletPointRepo;

impl WalletPointRepo {
    /// # 1. [REPOSITORY] - 创建积分账户并记录初始赠送交易
    /// * `desc`: `在同一 PostgreSQL 事务中幂等创建 POINT 账户与初始化流水`
    /// * `condition`: `POINT 不存在、用户ID无效或积分为负数时返回错误`
    pub async fn init_point_account(cmd: WalletPointInitCommand) -> Result<PointAccountInitRecord> {
        cmd.validate()?;

        let pool = pg_pool();
        Self::init_point_account_with_pool(&pool, cmd).await
    }

    ////////

    async fn init_point_account_with_pool(
        pool: &PgPool,
        cmd: WalletPointInitCommand,
    ) -> Result<PointAccountInitRecord> {
        let mut tx = pool.begin().await.context("开启积分账户事务失败")?;

        let currency_id: i16 = sqlx::query_scalar(
            r#"
            SELECT id
            FROM cola_wallet.wallet_currencies
            WHERE code = 'POINT' AND is_enabled = TRUE
            LIMIT 1
            "#,
        )
        .fetch_optional(&mut *tx)
        .await
        .context("查询 POINT 资产失败")?
        .ok_or_else(|| anyhow!("POINT 资产未配置或未启用"))?;

        let initial_points = Decimal::from(cmd.initial_points);

        let account_row = sqlx::query(
            r#"
            INSERT INTO cola_wallet.wallet_accounts
                (user_id, currency_id, balance, frozen_balance, total_income,
                 total_expense, status, version)
            VALUES ($1, $2, $3, 0, $3, 0, 1, 0)
            ON CONFLICT (user_id, currency_id) DO NOTHING
            RETURNING id, user_id, balance
            "#,
        )
        .bind(cmd.user_id)
        .bind(currency_id)
        .bind(initial_points)
        .fetch_optional(&mut *tx)
        .await
        .context("创建 POINT 账户失败")?;

        let (account_id, user_id, balance_decimal, is_new_account) = if let Some(row) = account_row
        {
            (
                row.get::<i64, _>("id"),
                row.get::<i64, _>("user_id"),
                row.get::<Decimal, _>("balance"),
                true,
            )
        } else {
            let row = sqlx::query(
                r#"
                SELECT id, user_id, balance
                FROM cola_wallet.wallet_accounts
                WHERE user_id = $1 AND currency_id = $2
                FOR UPDATE
                "#,
            )
            .bind(initial_points)
            .bind(currency_id)
            .fetch_one(&mut *tx)
            .await
            .context("查询已有 POINT 账户失败")?;

            (
                row.get::<i64, _>("id"),
                row.get::<i64, _>("user_id"),
                row.get::<Decimal, _>("balance"),
                false,
            )
        };

        let transaction_id = if is_new_account && cmd.initial_points > 0 {
            let row = sqlx::query(
                r#"
                INSERT INTO cola_wallet.wallet_transactions
                    (tx_no, user_id, wallet_id, currency_id, tx_type, tx_direction,
                     amount, balance_before, balance_after, fee, status,
                     idempotency_key, business_type, business_id, remark,
                     completed_at)
                VALUES ($1, $2, $3, $4, 'COIN_EARN', 'IN', $5, 0, $5, 0, 1,
                        $6, $7, $8, $9, CURRENT_TIMESTAMP)
                ON CONFLICT (idempotency_key) DO NOTHING
                RETURNING id
                "#,
            )
            .bind(format!("POINT-{}", cmd.idempotency_key))
            .bind(user_id)
            .bind(account_id)
            .bind(currency_id)
            .bind(Decimal::from(cmd.initial_points))
            .bind(&cmd.idempotency_key)
            .bind(&cmd.business_type)
            .bind(&cmd.business_id)
            .bind(&cmd.remark)
            .fetch_optional(&mut *tx)
            .await
            .context("保存初始积分交易失败")?;

            row.map(|item| item.get::<i64, _>("id"))
        } else {
            None
        };

        tx.commit().await.context("提交积分账户事务失败")?;

        Ok(PointAccountInitRecord {
            account_id,
            user_id,
            balance: balance_decimal
                .to_i64()
                .ok_or_else(|| anyhow!("POINT账户余额不是有效整数"))?,
            transaction_id,
            is_new_account,
        })
    }
}

//////// END
