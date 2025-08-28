use hyper::service::{make_service_fn, service_fn};
use hyper::server::Server;
use hyper::{Body, Method, Request, Response, StatusCode};
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path = req.uri().path();
    let method = req.method();
    
    match (method, path) {
        (&Method::GET, "/") => {
            let html = include_str!("index.html");
            Ok(Response::new(Body::from(html)))
        }
        (&Method::POST, "/") => {
            // Handle POST to root - just return OK for smuggling attack
            Ok(Response::new(Body::from("OK")))
        }
        (&Method::GET, "/admin") => {
            // Check for admin access - simplified check
            let headers = req.headers();
            let is_admin = headers.get("x-admin-access").is_some();
            
            if is_admin {
                let flag = env::var("FLAG").unwrap_or("FLAG{not_found}".to_string());
                let admin_html = include_str!("admin.html").replace("{{FLAG}}", &flag);
                Ok(Response::new(Body::from(admin_html)))
            } else {
                Ok(Response::builder()
                    .status(StatusCode::FORBIDDEN)
                    .body(Body::from("Access Denied: Admin access required"))
                    .unwrap())
            }
        }
        (&Method::POST, "/admin") => {
            // Internal admin endpoint - should only be accessible via smuggled request
            let flag = env::var("FLAG").unwrap_or("FLAG{not_found}".to_string());
            Ok(Response::new(Body::from(format!("FLAG: {}", flag))))
        }
        _ => {
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("Not Found"))
                .unwrap())
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle_request))
    });

    let addr: SocketAddr = ([0, 0, 0, 0], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Server running on http://0.0.0.0:3000");

    server.await?;
    Ok(())
}