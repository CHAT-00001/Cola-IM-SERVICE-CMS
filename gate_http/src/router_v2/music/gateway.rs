// gate_http/src/router_v2/music/gateway.rs
// 🌐 网关 - 可乐音乐 - 统一 API 转发
// 2026/8/23 02:00 Created.

////////

use crate::kits::response::IntoApi;
use crate::ping::ping;
use actix_web::{HttpMessage, HttpRequest, Responder, web};
use app_config::app_state::AppState;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::music::command::album::add::CreateMusicAlbumCmd;
use cola_data::music::command::music::new::MusicCreateCommand;
use cola_music::api::album::add::MusicAlbumAddApi;
use cola_music::api::music::add::MusicAddApi;
use std::time::Instant;

////////

/// # [ROUTER] - 音乐路由
/// * `desc`: `统一通过 music gateway 转发到 cola_music API`
pub fn music_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/music")
            .route("", web::get().to(ping))
            .route("/gateway", web::get().to(music_gateway))
            .route("/gateway", web::post().to(music_gateway)),
    );
}

////////

/// # [GATEWAY] - Body 命令提取
/// * `desc`: `兼容 Body 直接传命令和 { cmd: {...} } 包装格式`
fn extract_cmd<T: serde::de::DeserializeOwned>(
    request: &ApiGatewayRequest,
) -> Result<T, String> {
    let body = request
        .body
        .as_ref()
        .ok_or_else(|| "音乐请求缺少 JSON Body".to_string())?;
    let value = body.get("cmd").cloned().unwrap_or_else(|| body.clone());
    serde_json::from_value(value).map_err(|error| format!("音乐业务命令解析失败: {error}"))
}

////////

/// # [GATEWAY] - JSON 请求合并
/// * `desc`: `URL 为基础，Body JSON 字段覆盖 URL 字段`
fn merge_request(
    url_req: ApiGatewayRequest,
    body: &web::Bytes,
) -> Result<ApiGatewayRequest, String> {
    if body.is_empty() {
        return Ok(url_req.build());
    }

    let body_value: serde_json::Value = serde_json::from_slice(body)
        .map_err(|error| format!("[🌐 GATEWAY] - ❌️ 音乐 Body JSON 解析失败: {error}"))?;
    let mut body_req: ApiGatewayRequest = serde_json::from_value(body_value.clone())
        .map_err(|error| format!("[🌐 GATEWAY] - ❌️ 音乐 Body 通用参数解析失败: {error}"))?;
    body_req.body = Some(body_value);

    Ok(url_req.merge(body_req).build())
}

////////

/// # [GATEWAY] - 可乐音乐统一网关
/// * `desc`: `完成参数合并后直接转发到 cola_music API，不调用 Port`
pub async fn music_gateway(
    req: HttpRequest,
    url: web::Query<ApiGatewayRequest>,
    body: web::Bytes,
    state: web::Data<AppState>,
) -> impl Responder {
    let start = Instant::now();
    let uid = req.extensions().get::<i64>().copied().unwrap_or(1);
    let api_req = match merge_request(url.into_inner(), &body) {
        Ok(mut request) => {
            request.uid = Some(uid);
            request.build()
        }
        Err(error) => {
            return AppData::<()>::err(4001, error, None).finish(&req, start);
        }
    };
    match api_req.service.as_deref().unwrap_or_default() {
        "music.add" => {
            let command = match extract_cmd::<MusicCreateCommand>(&api_req) {
                Ok(command) => command,
                Err(error) => return AppData::<()>::err(4001, error, None).finish(&req, start),
            };
            MusicAddApi::api_add_music(uid, api_req.clone(), command, state.ctx.clone())
                .await
                .finish(&req, start)
        }
        "album.add" => {
            let command = match extract_cmd::<CreateMusicAlbumCmd>(&api_req) {
                Ok(mut command) => {
                    command.uid = uid;
                    command
                }
                Err(error) => return AppData::<()>::err(4001, error, None).finish(&req, start),
            };
            MusicAlbumAddApi::api_add_album(uid, command, state.ctx.clone())
                .await
                .finish(&req, start)
        }
        _ => AppData::<()>::err(
            2004,
            format!("[🌐 GATEWAY] - ⚠️ 未知音乐 service: {}", api_req.service.unwrap_or_default()),
            None,
        )
        .finish(&req, start),
    }
}

//////// END