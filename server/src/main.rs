use std::convert::Infallible;
use std::net::SocketAddr;
use std::env;
use hyper::{Body, Request, Response, Server, Method, StatusCode};
use hyper::service::{make_service_fn, service_fn};

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        // Rute untuk halaman utama
        (&Method::GET, "/") => {
            let html_content = tokio::fs::read_to_string("src/index.html").await.unwrap_or_else(|_| "File not found".to_string());
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/html")
                .body(html_content.into())
                .unwrap())
        }
        
        // Rute rahasia yang berisi flag
        (&Method::GET, "/secret") => {
            // Baca flag dari environment variable
            let flag = env::var("FLAG").unwrap_or_else(|_| "FLAG_NOT_FOUND".to_string());
            let mut html_content = tokio::fs::read_to_string("src/secret.html").await.unwrap_or_else(|_| "File not found".to_string());
            
            // Ganti placeholder dengan flag asli
            html_content = html_content.replace("{{FLAG}}", &flag);

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/html")
                .body(html_content.into())
                .unwrap())
        }

        // Rute lainnya akan menghasilkan 404
        _ => {
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("404 Not Found".into())
                .unwrap())
        }
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle_request))
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}