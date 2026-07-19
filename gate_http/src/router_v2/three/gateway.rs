// gate_http/src/router_v2/three/gateway.rs  -- THREE 网关
// 2026/6/18

//////

use crate::kits::response::IntoApi;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};
use cola_data::app::data::AppData;
use cola_data::auth::info::auth::AuthContext;
use cola_three::api::three_type::TypeApi;
use cola_three::api::three_vendor::VendorApi;
use cola_three::api::three_config::ConfigApi;
use cola_three::api::three_biz_binding::BindingApi;
use cola_three::model::command::three_type::TypeCommand;
use cola_three::model::command::three_vendor::VendorCommand;
use cola_three::model::command::three_config::ConfigCommand;
use cola_three::model::command::three_biz_binding::BindingCommand;
use serde::Deserialize;
use std::time::Instant;
use app_config::app_state::AppState;
use crate::ping::ping;
//////

/// # 统一的 Query 提取结构体
#[derive(Deserialize)]
pub struct ThreeGatewayQuery {
    pub service: String,
    pub action: Option<i16>,
    pub type_id: Option<i64>,
    pub vendor_id: Option<i64>,
    pub config_id: Option<i64>,
    pub code: Option<String>,
    pub biz_module: Option<String>,
    pub biz_type: Option<String>,
}

/// # [ROUTER] - THREE - 第三方服务路由器
pub fn three_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/three")
            .route("/", web::get().to(ping))
            .route("/gateway", web::post().to(three_gateway))
            .route("/gateway", web::get().to(three_gateway))
            .route("/gateway", web::post().to(three_gateway)),
    );
}

//////

/// # [GATEWAY] - 第三方服务管理网关
async fn three_gateway(
    req: HttpRequest,
    query: web::Query<ThreeGatewayQuery>,
    body: web::Bytes,
    state: web::Data<AppState>,
) -> impl Responder {

    let start = Instant::now();

    let _uid = match req.extensions().get::<i64>().copied() {
        Some(id) => id,
        None => 1,
    };

    let _auth = AuthContext {
        uid: _uid,
        access_token: String::new(),
        refresh_token: String::new(),
        device_id: String::new(),
        roles: vec![],
        is_anonymous: false,
    };

    // 获取 THREE Port
    let three = &state.ctx.three;

    match query.service.as_str() {

        ////////

        // 1001 类型添加
        "type_upsert" => {
            let cmd: TypeCommand = match serde_json::from_slice(&body) {
                Ok(c) => c,
                Err(e) => return AppData::<()>::err(400, format!("参数解析失败: {}", e), None).finish(&req, start),
            };
            TypeApi::upsert(three.r#type.as_ref(), cmd).await.finish(&req, start)
        }

        // 1002 类型列表
        "type_list" => {
            TypeApi::list(three.r#type.as_ref()).await.finish(&req, start)
        }

        // 1003 类型查询
        "type_find_by_code" => {
            let code = query.code.as_deref().unwrap_or("");
            TypeApi::find_by_code(three.r#type.as_ref(), code).await.finish(&req, start)
        }

        ////////

        // 5001 厂商添加
        "vendor_upsert" => {
            let cmd: VendorCommand = match serde_json::from_slice(&body) {
                Ok(c) => c,
                Err(e) => return AppData::<()>::err(400, format!("参数解析失败: {}", e), None).finish(&req, start),
            };
            VendorApi::upsert(three.vendor.as_ref(), cmd).await.finish(&req, start)
        }

        // 5002 厂商列表
        "vendor_list" => {
            VendorApi::list(three.vendor.as_ref()).await.finish(&req, start)
        }

        // 5003 厂商查询
        "vendor_find_by_code" => {
            let code = query.code.as_deref().unwrap_or("");
            VendorApi::find_by_code(three.vendor.as_ref(), code).await.finish(&req, start)
        }

        ////////

        // 2001 配置
        "config_upsert" => {
            let cmd: ConfigCommand = match serde_json::from_slice(&body) {
                Ok(c) => c,
                Err(e) => return AppData::<()>::err(400, format!("参数解析失败: {}", e), None).finish(&req, start),
            };
            ConfigApi::upsert(three.config.as_ref(), cmd).await.finish(&req, start)
        }


        // 2002 类型列表
        "config_list_by_type" => {
            let type_id = query.type_id.unwrap_or(0);
            ConfigApi::list_by_type(three.config.as_ref(), type_id).await.finish(&req, start)
        }

        // 2003 查找一个
        "config_find_by_id" => {
            let id = query.config_id.unwrap_or(0);
            ConfigApi::find_by_id(three.config.as_ref(), id).await.finish(&req, start)
        }

        // 2004 绑定
        "config_find_binded" => {
            let biz_module = query.biz_module.as_deref().unwrap_or("");
            let biz_type = query.biz_type.as_deref().unwrap_or("");
            ConfigApi::find_binded(three.config.as_ref(), biz_module, biz_type).await.finish(&req, start)
        }

        ////////

        // 4001 绑定添加
        "binding_upsert" => {
            let cmd: BindingCommand = match serde_json::from_slice(&body) {
                Ok(c) => c,
                Err(e) => return AppData::<()>::err(400, format!("参数解析失败: {}", e), None).finish(&req, start),
            };
            BindingApi::upsert(three.binding.as_ref(), cmd).await.finish(&req, start)
        }

        // 4002 绑定列表
        "binding_list" => {
            BindingApi::list(three.binding.as_ref()).await.finish(&req, start)
        }

        // 4003 绑定查询
        "binding_find_by_biz" => {
            let biz_module = query.biz_module.as_deref().unwrap_or("");
            let biz_type = query.biz_type.as_deref().unwrap_or("");
            BindingApi::find_by_biz(three.binding.as_ref(), biz_module, biz_type).await.finish(&req, start)
        }

        _ => AppData::<()>::err(400, format!("Unknown THREE service: {}", query.service), None).finish(&req, start),
    }
}


//////// END