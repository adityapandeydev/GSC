use axum::middleware::Next;
use axum::response::Response;
use axum::extract::Request;

pub async fn log_request(request: Request, next: Next) -> Response {
    println!("Incoming request: {} {}", request.method(), request.uri());
    next.run(request).await
}