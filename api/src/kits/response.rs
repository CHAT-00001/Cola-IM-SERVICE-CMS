// api/src/kits/page  -- 响应日志
// 2026-03-27 09:01

use data::app::data::AppData;

////////

pub trait IntoApi {
    // 明确返回 HttpResponse，不再使用 impl Responder 这种模糊的写法
    fn finish(self, req: &actix_web::HttpRequest, start: std::time::Instant) -> actix_web::HttpResponse;
}

impl<T: serde::Serialize> IntoApi for AppData<T> {
    fn finish(self, req: &actix_web::HttpRequest, start: std::time::Instant) -> actix_web::HttpResponse {
        let log_id = req.headers()
            .get("X-Log-ID")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("REQ_INTERNAL")
            .to_string();

        let mut final_res = self;

        // 注意：这里需要配合你 AppData 结构体的类型改动
        // 如果 AppData 里的字段还是 ()，赋值会报错，见下方的“同步改动”
        final_res.log_id = log_id;
        final_res.duration = format!("{:?}", start.elapsed());

        actix_web::HttpResponse::Ok().json(final_res)
    }
}