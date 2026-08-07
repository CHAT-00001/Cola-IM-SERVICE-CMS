// case/add  -- 可乐GIS - 用例层 - 购买
// 2026-07-07

//////

use anyhow::{Result, anyhow};
use tracing::{info, warn};
use cola_data::app::ctx::AppContext;
use cola_data::cola_gis::command::buy::PoiBuyCommand;

//////

/// # [USE CASE] - 购买 用例
pub struct BuyCase;

impl BuyCase {

    ////////

    /// # 1. [CASE] - 添加
    pub async fn case_add_poi_buy(
        uid: i64,
        poi_id: i64,
        cmd: PoiBuyCommand,
        ctx: &AppContext,
    ) -> Result<()> {

        // 1. 保存购买记录
        ctx.gis.buy
            .save_buy_record(uid, poi_id)
            .await
            .map_err(|e| anyhow!("添加购买记录失败: {}", e))?;

        Ok(())
    }

    ////////

    /// # 2. [CASE] - 删除
    pub async fn case_del_poi_buy(
        uid: i64,
        poi_id: i64,
        cmd: PoiBuyCommand,
        ctx: &AppContext,
    ) -> Result<()> {

        // 1. 删除购买记录
        ctx.gis.buy
            .del_buy_record(uid, poi_id)
            .await
            .map_err(|e| anyhow!("删除购买记录失败: {}", e))?;

        Ok(())
    }

}

////// END