// /info.rs  -- 
// 2026/6/10 07:28

////////

use async_trait::async_trait;

#[async_trait]
pub trait InfoPort : Send + Sync + 'static {
    //
    async fn get_info(
        &self,
        user_id: i64,
    ) -> anyhow::Result<()>;
}