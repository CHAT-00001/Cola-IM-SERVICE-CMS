// cola_video/port/cola_video/get.rs -- 端口 - VIDEO -  视频内容 - 获取端口
// 2026/8/5 00:00 Created.

////////

use cola_data::cola_video::info::video::VideoInfo;

////////

/// # [GET PORTS] -  获取
/// * `desc`: `获取视频`
#[async_trait::async_trait]
pub trait VideoGetPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 我的
    /// * `desc`: `▶ 可乐视频` - 获取我的视频列表信息
    async fn get_my_list(
        &self,
        uid: i64,                // UID
        keyword: Option<String>, // 关键词
        limit: i64,              // 数量
        offset: i64,             // 页码
        is_liked: bool,
    ) -> anyhow::Result<(Vec<VideoInfo>)>;

    ////////

    /// # 2. [PORT] - TA的
    /// * `desc`: `▶ 可乐视频` - 获取TA的视频列表信息
    async fn get_he_list(
        &self,
        uid: i64,                // UID
        user_id: i64,            // 用户 ID
        keyword: Option<String>, // 关键词
        limit: i64,              // 数量
        offset: i64,             // 页码
    ) -> anyhow::Result<(Vec<VideoInfo>)>;

    ////////

    /// # 3. [PORT] - 单个获取视频信息
    /// * `desc`: `COLA VIDEO` - 获取TA的视频列表信息
    async fn get_video_info_by_id(
        &self,
        uid: i64,      // UID
        video_id: i64, // 视频 ID
    ) -> anyhow::Result<(VideoInfo)>;

    ////////

    /// # 4. [PORT] - 批量获取视频信息
    /// * `desc`: `COLA VIDEO` - 获取TA的视频列表信息
    async fn get_video_infos_by_ids(
        &self,
        uid: i64,            // UID
        video_ids: Vec<i64>, // 视频 IDs
    ) -> anyhow::Result<(Vec<VideoInfo>)>;
}

//////// END
