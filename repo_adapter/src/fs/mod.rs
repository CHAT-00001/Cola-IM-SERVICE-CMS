// repo_adapter/src/fs/mod.rs -- 适配器 - FS - mod
// 2026/8/10 20:00 Updated.

////////

use port::fs::ColaFileStoagePort;
use std::sync::Arc;

////////
pub mod bucket;
pub mod cdn;
pub mod file;
pub mod media;
pub mod session; // 会话
pub mod upload; // 通用上传

////////

/// # [BUILD] - 构建 FS Port
/// * `desc`: 构建文件存储
pub fn build_cola_fs_port() -> ColaFileStoagePort {
    ColaFileStoagePort {
        bucket: bucket::build_fs_bucket_port(),
        cdn: cdn::build_fs_cdn_port(),
        file: file::build_fs_file_port(),
        media: media::build_auth_meida_port(),
        upload: Arc::new(upload::UploadSessionAdapter),
    }
}

//////// END
