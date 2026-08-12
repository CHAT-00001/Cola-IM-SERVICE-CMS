// cola_live/src/case/add  -- LIVE - 用例层 - 购买
// 2026/6/10 08:41

////////

use anyhow::{Result, anyhow};
use tracing::{info, warn};
use cola_data::cola_video::command::buy::VideoBuyCommand;
use port::app::ctx::AppContext;

////////

/// # [USE CASE] - 购买 用例
pub struct BuyCase;

impl BuyCase {

    ////////

    /// # 1. [CASE] - 添加
    pub async fn case_add_video_buy(
        uid: i64,
        video_id: i64,
        cmd: VideoBuyCommand,
        ctx: &AppContext,
    ) -> Result<()> {

        // 1. 保存购买记录
        ctx.video
            .buy
            .add
            .save_buy_record(uid, video_id)
            .await
            .map_err(|e| anyhow!("添加购买记录失败: {}", e))?;

        // 2. 校验


        // 3. 执行收藏

        Ok(())
    }

    ////////

    /// # 1. [CASE] - 删除
    pub async fn case_del_video_buy(
        uid: i64,
        video_id: i64,
        cmd: VideoBuyCommand,
        ctx: &AppContext,
    ) -> Result<()> {

        // 订单ID
        let id = 10083;

        // 1. 保存购买记录
        ctx.video
            .buy
            .del
            .single_delete(id)
            .await
            .map_err(|e| anyhow!("删除购买记录数失败: {}", e))?;

        // 2. 校验


        // 3. 执行收藏

        Ok(())
    }

}

//////// END
