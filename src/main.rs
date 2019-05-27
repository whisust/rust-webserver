extern crate actix_web;
extern crate listenfd;

use actix_web::*;
use bytes::Bytes;
use futures::stream::once;
use listenfd::ListenFd;

fn index(_req: &HttpRequest) -> HttpResponse {
    let body = once(Ok(Bytes::from_static(
        concat!("{\"msg\": \"", "\"}").as_bytes(),
    )));

    HttpResponse::Ok()
        .content_type("application/json")
        .body(Body::Streaming(Box::new(body)))
}

fn index_app(_req: &HttpRequest) -> &'static str {
    "Hello world!, from app"
}

fn main() {
    let mut listenfd = ListenFd::from_env();

    let mut server = server::new(|| vec![App::new().resource("/async", |r| r.f(index))]);

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind("127.0.0.1:8000").unwrap()
    };

    server.run();
}
