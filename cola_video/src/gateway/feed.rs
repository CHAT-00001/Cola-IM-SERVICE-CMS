// cola_video gateway/feed.rs  -- 数据流
// 2026/6/4 15:45 by wx: cestbon10080

////////

/// # [CODE] - 数据流
pub enum FeedCode {
    visited = 4000,   // 浏览记录
    hotlist = 4001,   // 热门列表
    recommend = 4002, // 为您推荐
    city = 4003,      // 同城
    nearby = 4004,    // 我附近的
    user = 4005,      // 用户主页
    publish = 4006,   // 我发布的
    following = 4007, // 关注的人的列表
    liked = 4008,     // 我点赞的
    collected = 4009, // 我收藏的
    // ...
}
