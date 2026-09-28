// cola_video/src/guard/error.rs -- VIDEO - GUARD - 守卫错误
// 2026/9/29 Created.

////////

/// # [GUARD] - 守卫拒绝原因
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuardError {
    VideoNotFound,
    PermissionDenied,
    CommentClosed,
    AuthorOnly,
    MutualFollowRequired,
    FollowerRequired,
    RequesterBlocked,
    AuthorBlocked,
}

//////// END
