// cola_music/src/case/music/add.rs  -- MUSIC - 用例层 - 音乐 - 发布用例
// 2026-07-08 14:20

////////

use anyhow::Result;
use cola_data::cola_fs::rick_check;
use cola_data::music::command::album::add::CreateMusicAlbumCmd;
use cola_data::music::command::album::edit::UpdateMusicAlbumCmd;
use cola_data::music::vo::album::MusicAlbumSingleResponse;
use port::app::ctx::AppContext;
use tracing::info;

////////

/// # [CASE] - 音乐 发布 用例
/// * `desc`: `可乐音乐 - 专辑发布用例`
pub struct MusicAlbumAddCase;

impl MusicAlbumAddCase {
    //

    ////////

    /// # 1. [CASE] - 创建专辑
    pub async fn case_add_album(
        uid: i64,                 //  操作者 ID
        cmd: CreateMusicAlbumCmd, // 创建命令
        ctx: AppContext,          // 应用上下文
    ) -> Result<MusicAlbumSingleResponse, anyhow::Error> {
        let check_text = format!("{:?} {:?}", cmd.name, cmd.description);

        let visibility = rick_check(check_text).await;

        ctx.music
            .album
            .add
            .create_album(uid, cmd, visibility)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 ALBUM CASE]: ❌️ 音乐专辑创建失败: {}", e))?;

        Err(anyhow::anyhow!("[🤐 ALBUM CASE]: ❌️ 专辑响应组装尚未完成"))
    }

    ////////

    /// # 2. [CASE] - 编辑专辑
    pub async fn case_edit_album(
        uid: i64,                 // 操作者 ID
        album_id: i64,            // 专辑 ID
        cmd: UpdateMusicAlbumCmd, // 更新命令
        ctx: AppContext,          // 应用上下文
    ) -> Result<MusicAlbumSingleResponse, anyhow::Error> {
        let check_text = format!("{:?} {:?}", cmd.name, cmd.description);

        let visibility = rick_check(check_text).await;

        ctx.music
            .album
            .add
            .update_album(uid, album_id, cmd, visibility)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 ALBUM CASE]: ❌️ 音乐专辑编辑失败: {}", e))?;

        Err(anyhow::anyhow!("[🤐 ALBUM CASE]: ❌️ 专辑响应组装尚未完成"))
    }
}

//////// END
