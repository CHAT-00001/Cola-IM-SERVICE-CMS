// cola_video/src/guard/comment.rs -- VIDEO - GUARD - 评论策略
// 2026/9/29 Created.

////////

use super::error::GuardError;
use super::relation::AuthorRelation;

////////

/// # [GUARD] - 评论权限等级
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i16)]
pub enum CommentPermission {
    Closed = 0,
    AuthorOnly = 1,
    MutualFollowOnly = 2,
    FollowerOnly = 3,
    Everyone = 4,
}

impl TryFrom<i16> for CommentPermission {
    type Error = GuardError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Closed),
            1 => Ok(Self::AuthorOnly),
            2 => Ok(Self::MutualFollowOnly),
            3 => Ok(Self::FollowerOnly),
            4 => Ok(Self::Everyone),
            _ => Err(GuardError::PermissionDenied),
        }
    }
}

////////

/// # 1. [GUARD] - 判断评论权限
/// * `desc`: `纯函数，只消费视频作者关系与评论权限，不访问数据库或缓存`
pub fn can_comment(
    requester_uid: i64,            // 请求者 UID
    author_uid: i64,               // 视频作者 UID
    permission: CommentPermission, // 视频评论权限
    relation: AuthorRelation,      // 请求者与作者关系
) -> Result<(), GuardError> {
    if requester_uid <= 0 || author_uid <= 0 {
        return Err(GuardError::VideoNotFound);
    }
    if relation.requester_blocked_author {
        return Err(GuardError::RequesterBlocked);
    }
    if relation.author_blocked_requester {
        return Err(GuardError::AuthorBlocked);
    }

    match permission {
        CommentPermission::Closed => Err(GuardError::CommentClosed),
        CommentPermission::AuthorOnly => {
            if requester_uid == author_uid {
                Ok(())
            } else {
                Err(GuardError::AuthorOnly)
            }
        }
        CommentPermission::MutualFollowOnly => {
            if requester_uid == author_uid || relation.is_mutual_follow() {
                Ok(())
            } else {
                Err(GuardError::MutualFollowRequired)
            }
        }
        CommentPermission::FollowerOnly => {
            if requester_uid == author_uid || relation.requester_follows_author {
                Ok(())
            } else {
                Err(GuardError::FollowerRequired)
            }
        }
        CommentPermission::Everyone => Ok(()),
    }
}

//////// END
