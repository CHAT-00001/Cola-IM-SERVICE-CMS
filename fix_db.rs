// 临时 DB 修复脚本: 给 video 表添加缺失列
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "postgres://postgres:123456@127.0.0.1:5432/live_2026";
    let pool = PgPool::connect(database_url).await?;

    let sql = r#"
        ALTER TABLE video ADD COLUMN IF NOT EXISTS channel_id SMALLINT NOT NULL DEFAULT 0;
        ALTER TABLE video ADD COLUMN IF NOT EXISTS thumbnail VARCHAR(500) DEFAULT NULL;
        ALTER TABLE video ADD COLUMN IF NOT EXISTS cover_url VARCHAR(500) DEFAULT NULL;
        ALTER TABLE video ADD COLUMN IF NOT EXISTS dislike INTEGER NOT NULL DEFAULT 0;
        ALTER TABLE video ADD COLUMN IF NOT EXISTS danmakus INTEGER NOT NULL DEFAULT 0;
        ALTER TABLE video ADD COLUMN IF NOT EXISTS recommends INTEGER NOT NULL DEFAULT 0;
        ALTER TABLE video ADD COLUMN IF NOT EXISTS is_public BOOLEAN DEFAULT TRUE;
        ALTER TABLE video ADD COLUMN IF NOT EXISTS is_del SMALLINT NOT NULL DEFAULT 0;
        ALTER TABLE video ADD COLUMN IF NOT EXISTS collect_perm SMALLINT NOT NULL DEFAULT 5;
        ALTER TABLE video ADD COLUMN IF NOT EXISTS buy_perm SMALLINT NOT NULL DEFAULT 5;
        ALTER TABLE video ADD COLUMN IF NOT EXISTS sync_at BIGINT DEFAULT NULL;
        ALTER TABLE video ADD COLUMN IF NOT EXISTS del_time BIGINT DEFAULT NULL;
        ALTER TABLE video ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMP DEFAULT NULL;
        ALTER TABLE video ADD COLUMN IF NOT EXISTS danmaku_perm SMALLINT NOT NULL DEFAULT 5;
    "#;

    for statement in sql.split(';').map(|s| s.trim()).filter(|s| !s.is_empty()) {
        println!("Executing: {}", &statement[..statement.len().min(80)]);
        sqlx::query(statement).execute(&pool).await?;
    }

    println!("✅ All columns added successfully!");
    Ok(())
}