use actix_web::{dev::{Transform, Service, ServiceRequest, ServiceResponse}, web, Error, HttpMessage}; 
use std::rc::Rc; 
use std::future::Ready; 
use jsonwebtoken::{Algorithm}; 
use crate::utils::Claims; 
use crate::AppState;

// JWT验证中间件
pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        std::future::ready(Ok(AuthMiddlewareService { service: Rc::new(service) }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = actix_web::dev::ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: actix_web::dev::ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            // 获取应用状态
            let state = req.app_data::<web::Data<AppState>>()
                .ok_or_else(|| {
                    actix_web::error::ErrorInternalServerError("Failed to get app state".to_string())
                })?;

            // 从请求头中获取Authorization令牌
            let auth_header = req.headers().get(actix_web::http::header::AUTHORIZATION)
                .map(|h| h.to_str().unwrap_or(""))
                .unwrap_or("");

            // 检查令牌格式是否正确
            if !auth_header.starts_with("Bearer ") {
                return Err(actix_web::error::ErrorUnauthorized("Auth fail, Please relgin".to_string()));
            }

            // 提取令牌
            let token = auth_header.trim_start_matches("Bearer ");

            // 验证令牌
            let decoded = jsonwebtoken::decode::<Claims>(
                token,
                &jsonwebtoken::DecodingKey::from_secret(state.config.jwt_secret.as_bytes()),
                &jsonwebtoken::Validation::new(Algorithm::HS256),
            ).map_err(|e| {
                log::error!("JWT validation error: {:?}", e);
                actix_web::error::ErrorUnauthorized("Invalid or expired token".to_string())
            })?;

            // 将用户ID添加到请求扩展中
            req.extensions_mut().insert(decoded.claims.sub);

            // 继续处理请求
            service.call(req).await
        })
    }
}