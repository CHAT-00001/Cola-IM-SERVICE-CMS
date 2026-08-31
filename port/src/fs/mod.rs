// port/src/fs/mod.rs -- 端口 - FS - mod
// 2026/6/10 07:45

////////

use crate::auth::session::SessionPort;
use crate::fs::bucket::FsBucketPort;
use crate::fs::cdn::FsCdnPort;
use crate::fs::file::FsFilePort;
use crate::fs::media::FsMediaPort;
use crate::fs::upload::UploadSessionPort;
use std::sync::Arc;

////////
pub mod bucket;
pub mod cdn;
pub mod file;
pub mod media;
pub mod session; // 登录会话
pub mod upload; // 通用上传

////////

/// # AUTH 上下文模型
#[derive(Clone)]
pub struct AuthServicePorts {
    /// 会话校验端口（Token 验证 → SessionContext）
    pub session: Arc<dyn SessionPort + Send + Sync + 'static>,
}

/// # [COLA FS PORTS] - 验证
/// * `desc`: `FS - Cola FS Service Port`
#[derive(Clone)]
pub struct ColaFileStoagePort {
    pub bucket: FsBucketPort,                                       // 存储桶
    pub cdn: FsCdnPort,                                             // CDN
    pub file: FsFilePort,                                           // 文件
    pub media: FsMediaPort,                                         // 媒体
    pub upload: Arc<dyn UploadSessionPort + Send + Sync + 'static>, // 通用上传
}

//////// END
