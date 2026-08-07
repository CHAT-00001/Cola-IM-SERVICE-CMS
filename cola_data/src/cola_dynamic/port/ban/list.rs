// cola_video/port/ban/list.rs
// 视频 - port - 封禁 - 列表
// 2026/8/5 00:07 Created.

////////

use crate::cola_video::command::danmaku::DanmakuCommand;
use crate::cola_video::info::danmaku::DanmakuInfo;

////////

/// # [ADD SERVICE] - 发布
/// * `desc`: `弹幕发布服务`
#[async_trait::async_trait]
pub trait DanmakuAddService: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 我的
    /// * `desc`: `获取我的弹幕记录`
    async fn get_my_record(
        &self,
        uid: i64,    // UID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<DanmakuInfo>)>;

    ////////

    /// # 2. [PORT] - 她的
    /// * `desc`: `获取TA的弹幕记录`
    async fn get_he_record(
        &self,
        uid: i64,    // UID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<DanmakuInfo>)>;

    ////////

    /// # 3. [PORT] - 视频的
    /// * `desc`: `获取video的弹幕记录`
    async fn get_new_record(
        &self,
        uid: i64,      // UID
        video_id: i64, // 视频ID
        limit: i64,    // 数量
        offset: i64,   // 页码
    ) -> anyhow::Result<(Vec<DanmakuInfo>)>;

    ////////

    /// # 2. [PORT] - 更新
    /// * `desc`: `更新弹幕记录`
    async fn edit_danmaku_record(
        &self,
        uid: i64,            // UID
        danmaku_id: i64,     // 弹幕ID
        cmd: DanmakuCommand, // 命令
    ) -> anyhow::Result<()>;

    ////////

    /// # 3. [PORT] - 删除
    async fn del_danmaku_record(&self, uid: i64, danmaku_id: i64) -> anyhow::Result<()>;

    ////////

    /// # 4. [PORT] - 批量删除
    async fn del_danmakus_record(&self, uid: i64, danmaku_ids: Vec<i64>) -> anyhow::Result<()>;

    ////////

    /// # 5. [PORT] - 获取视频ID的弹幕
    async fn get_danmaku_by_video_id(
        &self,
        uid: i64,
        video_id: i64,
        play_time: i32,
        qty: i32,
    ) -> anyhow::Result<(Vec<DanmakuInfo>, i64)>;

    ////////

    /// # 6. [PORT] - 获取用户ID的弹幕
    async fn get_danmaku_by_user_id(
        &self,
        uid: i64,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<(Vec<DanmakuInfo>, i64)>;
}

//////// END