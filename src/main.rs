
type HttpRequest = hyper::Request<hyper::Body>;
type HttpResponse = hyper::Response<hyper::Body>;
type HttpResult = Result<HttpResponse, hyper::Error>;
type MainError = Box<dyn std::error::Error + Send + Sync>;
type MainResult = Result<(), MainError>;

#[tokio::main]
pub async fn main() -> MainResult {
    let addr = ([127, 0, 0, 1], 3000).into();
    let service = hyper::service::make_service_fn(|_| async {
        Ok::<_, hyper::Error>(hyper::service::service_fn(handler))
    });
    let server = hyper::Server::bind(&addr).serve(service);
    println!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}

async fn handler(req: HttpRequest) -> HttpResult {
    let (parts, body) = req.into_parts();
    println!("HTTP {} {} {:?}", parts.method, parts.uri, parts.headers);
    let mut res = HttpResponse::new(hyper::Body::empty());
    if parts.method == &hyper::Method::PUT {
        *res.body_mut() = body;
    }
    Ok(res)
}
