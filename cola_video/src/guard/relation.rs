// cola_video/src/guard/relation.rs -- VIDEO - GUARD - 用户与作者关系
// 2026/9/29 Created.

////////

use anyhow::Result;
use port::app::ctx::AppContext;

////////

/// # [GUARD] - 请求者与视频作者的关系事实
/// * `desc`: `只描述关系，不包含任何具体业务权限判断`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AuthorRelation {
    pub requester_follows_author: bool, // 请求者是否关注作者
    pub author_follows_requester: bool, // 作者是否关注请求者
    pub requester_blocked_author: bool, // 请求者是否拉黑作者
    pub author_blocked_requester: bool, // 作者是否拉黑请求者
}

impl AuthorRelation {
    ////////

    /// # 1. [GUARD] - 双向关注
    pub const fn is_mutual_follow(self) -> bool {
        self.requester_follows_author && self.author_follows_requester
    }

    ////////

    /// # 2. [GUARD] - 通过 AppContext 加载请求者与作者关系
    /// * `desc`: `统一从 USER Port 查询关注与黑名单关系`
    pub async fn from_context(
        requester_uid: i64, // 请求者 UID
        author_uid: i64,    // 作者 UID
        ctx: &AppContext,   // 应用上下文
    ) -> Result<Self> {
        if requester_uid <= 0 || author_uid <= 0 || requester_uid == author_uid {
            return Ok(Self::default());
        }

        let requester_follows_author = ctx
            .user
            .follow
            .check
            .is_followed(requester_uid, author_uid)
            .await?;
        let author_follows_requester = ctx
            .user
            .follow
            .check
            .is_followed(author_uid, requester_uid)
            .await?;
        let requester_blocked_author = ctx
            .user
            .black
            .check
            .is_blacked(requester_uid, author_uid)
            .await?;
        let author_blocked_requester = ctx
            .user
            .black
            .check
            .is_blacked(author_uid, requester_uid)
            .await?;

        Ok(Self {
            requester_follows_author,
            author_follows_requester,
            requester_blocked_author,
            author_blocked_requester,
        })
    }
}

//////// END
