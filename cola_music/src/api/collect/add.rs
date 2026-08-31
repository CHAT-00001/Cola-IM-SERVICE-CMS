// music/src/api/collect/add.rs -- 可乐音乐 - 接口层 - 收藏 - 发布
// 2026-07-08 14:52 Created.

////////

use crate::case::collect::add::MusicCollectAddCase;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::app::query::ApiGatewayRequest;
use port::app::ctx::AppContext;

////////

/// # [API] - 发布 接口
/// * `desc`: `可乐音乐 - 收藏发布接口`
pub struct MusicCollectAddApi {
    music_id: i64,
}

impl MusicCollectAddApi {
    //

    ////////

    /// # 1. [API HANDLER] - 添加收藏
    pub async fn api_add_collect(
        uid: i64,               // 操作者 ID
        url: ApiGatewayRequest, // 网关请求参数
        ctx: AppContext,        // 应用上下文
    ) -> AppData<bool> {
        // 音乐 ID
        let music_id = url.id;
        // 专辑 ID (这里将 i64 转回 Option<i64> 传递给 case)
        let album_id = if url.album_id.unwrap_or_default() == 0 {
            None
        } else {
            url.album_id
        };

        // 检查音乐状态（注意：这里如果是异步方法，可能需要加上 .await，根据你的实际架构调整）
        // let state = ctx.music.music.check.state(music_id).await;

        // 调用 Case 层，接收 bool 状态
        match MusicCollectAddCase::case_add_collect(uid, music_id, album_id, ctx).await {
            Ok(is_changed) => {
                let msg = if is_changed {
                    "收藏音乐成功"
                } else {
                    "已经收藏过了"
                };
                AppData::ok(true).with_msg(msg)
            }
            Err(e) => AppData::err(
                error::INTERNAL_ERROR,
                format!("收藏音乐失败: {:?}", e),
                None,
            ),
        }
    }

    ////////

    /// # 2. [API HANDLER] - 取消收藏
    pub async fn api_del_collect(
        uid: i64,               // 操作者 ID
        url: ApiGatewayRequest, // 网关请求参数
        ctx: AppContext,        // 应用上下文
    ) -> AppData<bool> {
        let music_id = url.music_id;

        // 调用 Case 层，接收 bool 状态
        match MusicCollectAddCase::case_del_collect(uid, music_id, ctx).await {
            Ok(is_changed) => {
                let msg = if is_changed {
                    "取消收藏成功"
                } else {
                    "本来就没有收藏记录"
                };
                AppData::ok(false).with_msg(msg)
            }
            Err(e) => AppData::err(
                error::INTERNAL_ERROR,
                format!("取消收藏失败: {:?}", e),
                None,
            ),
        }
    }
}

//////// END
