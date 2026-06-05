// cola_video/src/video/biz/del  -- 核心 - 短视频 - 业务层 - 删除
// 2026/5/20 03:25 by wx: cestbon10080
// * 8个流程
// * -----

////////

use crate::video::port::del::DelPort;
use anyhow::{Context, Result};
use tracing::{info, warn};

////////

/// # 1. [LOGIC] - 删除 视频
pub async fn logic_del_video(
    uid: i64,
    video_id: i64,
    del_port: &dyn DelPort,
) -> Result<bool> {
    del_port
        .delete_video_and_update_stat(uid, video_id)
        .await
        .map_err(|e| anyhow::anyhow!("BIZ: 视频删除操作失败: {}", e))?;

    info!("BIZ - 视频删除成功: uid={}, video_id={}", uid, video_id);
    Ok(true)
}

////////

/// # 2. [LOGIC] - 删除 评论
pub async fn logic_del_comment(
    uid: i64,
    comment_id: i64,
    del_port: &dyn DelPort,
) -> Result<bool> {
    del_port
        .delete_comment_and_update_stat(uid, comment_id)
        .await
        .map_err(|e| anyhow::anyhow!("BIZ: 评论删除操作失败: {}", e))?;

    info!("BIZ - 评论删除成功: uid={}, comment_id={}", uid, comment_id);
    Ok(true)
}

////////

/// # 3. [LOGIC] - 删除 弹幕
pub async fn logic_del_danmaku(
    uid: i64,
    danmaku_id: i64,
    del_port: &dyn DelPort,
) -> Result<bool> {
    del_port
        .delete_danmaku_and_update_stat(uid, danmaku_id)
        .await
        .map_err(|e| anyhow::anyhow!("BIZ: 弹幕删除操作失败: {}", e))?;

    info!("BIZ - 弹幕删除成功: uid={}, danmaku_id={}", uid, danmaku_id);
    Ok(true)
}

////////

/// # 4. [LOGIC] - 删除 收藏
pub async fn logic_del_collect(
    uid: i64,
    collect_id: i64,
    del_port: &dyn DelPort,
) -> Result<bool> {
    del_port
        .delete_collect_and_update_stat(uid, collect_id)
        .await
        .map_err(|e| anyhow::anyhow!("BIZ: 收藏删除操作失败: {}", e))?;

    info!("BIZ - 收藏删除成功: uid={}, collect_id={}", uid, collect_id);
    Ok(true)
}

////////

/// # 5. [LOGIC] - 删除 推荐
pub async fn logic_del_recommend(
    uid: i64,
    recommend_id: i64,
    del_port: &dyn DelPort,
) -> Result<bool> {
    del_port
        .delete_recommend_and_update_stat(uid, recommend_id)
        .await
        .map_err(|e| anyhow::anyhow!("BIZ: 推荐删除操作失败: {}", e))?;

    info!("BIZ - 推荐删除成功: uid={}, recommend_id={}", uid, recommend_id);
    Ok(true)
}

////////

/// # 6. [LOGIC] - 删除 购买记录
pub async fn logic_del_buy(
    uid: i64,
    video_id: i64,
    del_port: &dyn DelPort,
) -> Result<bool> {
    del_port
        .delete_buy_and_update_stat(uid, video_id)
        .await
        .map_err(|e| anyhow::anyhow!("BIZ: 购买记录删除操作失败: {}", e))?;

    info!("BIZ - 购买记录删除成功: uid={}, video_id={}", uid, video_id);
    Ok(true)
}

////////

/// # 7. [LOGIC] - 删除 浏览记录
pub async fn logic_del_visited(
    uid: i64,
    video_id: i64,
    del_port: &dyn DelPort,
) -> Result<bool> {
    del_port
        .delete_buy_and_update_stat(uid, video_id)
        .await
        .map_err(|e| anyhow::anyhow!("BIZ: 浏览记录删除操作失败: {}", e))?;

    info!("BIZ - 浏览记录删除成功: uid={}, video_id={}", uid, video_id);
    Ok(true)
}

////////

/// # 7. [LOGIC] - 删除 上热门记录
pub async fn logic_del_hotlist(
    uid: i64,
    video_id: i64,
    del_port: &dyn DelPort,
) -> Result<bool> {
    del_port
        .delete_hotlist_and_update_stat(uid, video_id)
        .await
        .map_err(|e| anyhow::anyhow!("BIZ: 热门记录删除操作失败: {}", e))?;

    info!("BIZ - 热门记录删除成功: uid={}, video_id={}", uid, video_id);
    Ok(true)
}

//////// END
