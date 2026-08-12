// cola_live/src/case/danmaku/get
// LIVE - 用例层 - 弹幕 - 发布
// 2026/8/12 04:44 Created.

////////

use crate::assembler::danmaku::build_danmaku_list_response;
use crate::model::vo::danmaku::DanmakuListResponse;
use anyhow::Result;
use cola_data::app::request::ApiUrlParamsQuery;
use port::app::ctx::AppContext;

////////

/// # [CASE] - 弹幕 获取
/// * `DESC`: `LIVE` - `弹幕获取用例`
pub struct DanmakuGetCase;

////////
impl DanmakuGetCase {
    //

    ////////

    /// # 2. [CASE] - 浏览视频弹幕列表
    pub async fn case_video_danmakus(
        uid: i64,
        video_id: i64,
        play_time: i32,
        _time_window: i32,
        qty: i64, // 💡 顺手把 qty 接进来，别写死 20 了
        ctx: &AppContext,
    ) -> Result<DanmakuListResponse> {
        // 1. 调用 Repo 端口，稳稳接住 (Vec<DanmakuInfo>, total)
        let (danmaku_list, total) = ctx
            .video
            .danmaku
            .get
            .get_danmaku_by_video_id(uid, video_id, play_time, qty as i32)
            .await?;

        // 2. 准备组装器要的标识参数
        let current_uid = if uid > 0 { Some(uid) } else { None };
        let video_author_id = 0i64; // 💡 预留：未来可以从视频服务获取真实的作者 ID

        // 3. ✅ 完美契合：6 个参数一字排开，数据流清清爽爽
        let response = build_danmaku_list_response(
            danmaku_list,
            current_uid,
            video_author_id,
            1,     // page 默认第一页（或者从别处扩展传入）
            qty,   // 动态传入前端请求的数量
            total, // 数据库真实的总条数
        )
        .await?;

        // 4. 返回视图
        Ok(response)
    }

    ////////

    /// # 1. [CASE] - 浏览用户弹幕列表
    pub async fn case_user_danmakus(
        uid: i64,
        query: ApiUrlParamsQuery,
        ctx: &AppContext,
    ) -> Result<DanmakuListResponse> {
        // 数量 来源 分页限制
        let qty = query.limit;

        // 1. 调用 Repo 端口，稳稳接住 (Vec<DanmakuInfo>, total)
        let (danmaku_list, total) = ctx
            .video
            .danmaku
            .get
            .get_danmaku_by_user_id(uid, query.offset, query.limit)
            .await?;

        // 2. 准备组装器要的标识参数
        let current_uid = if uid > 0 { Some(uid) } else { None };
        let video_author_id = 0i64; // 💡 预留：未来可以从视频服务获取真实的作者 ID

        // 3. ✅ 完美契合：6 个参数一字排开，数据流清清爽爽
        let response = build_danmaku_list_response(
            danmaku_list,
            current_uid,
            video_author_id,
            1,     // page 默认第一页（或者从别处扩展传入）
            qty,   // 动态传入前端请求的数量
            total, // 数据库真实的总条数
        )
        .await?;

        // 4. 返回视图
        Ok(response)
    }
}

//////// END
