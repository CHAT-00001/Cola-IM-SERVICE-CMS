// cola_video/src/guard/mod.rs -- VIDEO - GUARD - 守卫模块
// 2026/9/29 04:51 Created.

////////

pub mod comment;
pub mod context;
pub mod error;
pub mod relation;

pub use comment::{CommentPermission, can_comment};
pub use context::VideoGuardContext;
pub use error::GuardError;
pub use relation::AuthorRelation;

//////// END
