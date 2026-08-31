// repo_adapter/src/cola_im/contact_request.rs -- 🔌 适配器 - 可乐IM - 联系人  - 添加请求
// 2026-07-07 14:20 Created.

////////
use async_trait::async_trait;
use cola_data::cola_im::command::contact_request::ContactRequestCommand;
use cola_data::cola_im::info::contact_request::ContactRequestInfo;
use port::cola_im::contact_request::ContactRequestRepo;

////////

/// # [ADAPTER] - 联系人请求
/// * `desc`: `可乐IM - 联系人请求适配器`
pub struct ContactRequestPortAdapter;

#[async_trait]
impl ContactRequestRepo for ContactRequestPortAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 发送请求
    async fn send_request(
        &self,
        _uid: i64,                   // UID
        _cmd: ContactRequestCommand, // 命令
    ) -> anyhow::Result<()> {
        Ok(())
    }

    ////////

    /// # 2. [ADAPTER] - 同意请求
    async fn accept_request(
        &self,
        _uid: i64,        // UID
        _request_id: i64, // 请求 ID
    ) -> anyhow::Result<()> {
        Ok(())
    }

    ////////

    /// # 3. [ADAPTER] - 拒绝请求
    async fn refuse_request(
        &self,
        _uid: i64,        // UID
        _request_id: i64, // 请求 ID
        _replay: Option<String>,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    ////////

    /// # 4. [ADAPTER] - 请求列表
    async fn list_requests(
        &self,
        _uid: i64,    // UID
        _offset: i64, // 页码
        _limit: i64,  // 数量
    ) -> anyhow::Result<Vec<ContactRequestInfo>> {
        Ok(vec![])
    }
}

//////// END
