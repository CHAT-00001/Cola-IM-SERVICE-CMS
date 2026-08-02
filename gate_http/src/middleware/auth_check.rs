// src/middleware/auth_check.rs - jwt 验证
// 2026-01-16 13:45:00
// 2026/8/1 重构：统一使用 cola_auth::kits::token 中的密钥与载荷结构

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use cola_auth::kits::token::{JwtClaims, kit_get_jwt_secret};
use futures_util::future::{ready, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::rc::Rc;

////////

pub struct JwtAuth;

impl<S, B> Transform<S, ServiceRequest> for JwtAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtAuthMiddleware {
            service: Rc::new(service),
        }))
    }
}

////////

pub struct JwtAuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = Rc::clone(&self.service);

        Box::pin(async move {
            // 1. 获取 Authorization Header
            if let Some(auth_header) = req.headers().get("Authorization") {
                if let Ok(auth_str) = auth_header.to_str() {
                    if auth_str.starts_with("Bearer ") {
                        let token = &auth_str[7..];

                        // 2. 校验 JWT：使用与生成端完全一致的密钥（cola_auth 统一管理）
                        let secret = kit_get_jwt_secret();
                        let decoding_key = DecodingKey::from_secret(&secret);
                        let validation = Validation::default();

                        if let Ok(token_data) = decode::<JwtClaims>(token, &decoding_key, &validation) {
                            // 3. 【核心】验证成功，将 uid 注入 extensions
                            // 后续 ApiRequest 就能通过 req.extensions().get::<i64>() 拿到它
                            req.extensions_mut().insert(token_data.claims.sub);
                        } else {
                            // 验证失败放行，让业务层判断是否为游客
                            // tracing::warn!("JWT 验证失败: {}", token);
                        }
                    }
                }
            }

            // 继续执行后续逻辑（即使没 Token 也放行，因为 ApiRequest 会处理 Option<uid>）
            let res = srv.call(req).await?;
            Ok(res)
        })
    }
}

//////// END