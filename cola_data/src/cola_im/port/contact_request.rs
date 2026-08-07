// cola_data/src/cola_im/port/contact_request.rs  -- IM - port - 联系人请求
// 2026-07-07

use crate::cola_im::command::contact_request::ContactRequestCommand;
use crate::cola_im::info::contact_request::ContactRequestInfo;

#[async_trait::async_trait]
pub trait ContactRequestRepo: Send + Sync {
    /// 发送添加请求
    async fn send_request(&self, uid: i64, cmd: ContactRequestCommand) -> anyhow::Result<()>;

    /// 同意请求
    async fn accept_request(&self, uid: i64, request_id: i64) -> anyhow::Result<()>;

    /// 拒绝请求
    async fn refuse_request(&self, uid: i64, request_id: i64, replay: Option<String>) -> anyhow::Result<()>;

    /// 获取请求列表
    async fn list_requests(&self, uid: i64, offset: i64, limit: i64) -> anyhow::Result<Vec<ContactRequestInfo>>;
}