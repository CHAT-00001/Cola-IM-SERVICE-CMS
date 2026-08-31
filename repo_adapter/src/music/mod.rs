// repo_adapter/src/music/mod.rs -- 🔌 适配器 - 可乐音乐 - mod
// 2026/8/23 00:20 Created.

////////

use port::cola_music::ColaMusicPort;

////////

pub mod album; // 专辑
pub mod collect; // 收藏
pub mod content; // 音乐内容
pub mod like; // 点赞
pub mod user; // 用户
pub mod view; // 浏览(预设)

////////

/// # [BUILD] - 构建音乐 Port
/// * `desc`: `按业务子模块分别装配音乐端口`
pub fn build_music_port() -> ColaMusicPort {
    ColaMusicPort {
        album: album::build_music_album_port(),       // 专辑
        collect: collect::build_music_collect_port(), // 收藏
        music: content::build_music_content_port(),   // 音乐
        like: like::build_music_like_port(),          // 点赞
        user: user::build_music_user_port(),          // 用户
        view: view::build_view_port(),                // 浏览
    }
}

//////// END
