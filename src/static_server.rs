use hyper_staticfile::Static;
use hyper::service::{make_service_fn, service_fn};
use std::path::Path;

pub async fn static_serve() {
    let static_ = Static::new(Path::new("ui/build/"));

    let make_service = make_service_fn(|_| {
        let static_ = static_.clone();
        async { Ok::<_, hyper::Error>(service_fn(move |req| {
            println!("{:?}", req);
            static_.clone().serve(req)
        })) }
    });

    let addr = ([127, 0, 0, 1], 3300).into();
    let server = hyper::Server::bind(&addr).serve(make_service);
    eprintln!("Doc server running on http://{}/", addr);
    server.await.expect("Server failed");
}
