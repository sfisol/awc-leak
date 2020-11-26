use actix_web::{
    client::{ClientBuilder, Connector},
    rt, web, App, HttpServer
};
use std::{
    io,
    thread,
    str::from_utf8,
    include_bytes,
    time::Duration,
};

use openssl::ssl::{SslConnector, SslMethod, SslAcceptor, SslFiletype, SslVerifyMode};
use procinfo::pid::stat_self;

fn run_server() -> std::io::Result<()> {
    let mut sys = rt::System::new("test");

    // load ssl keys
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("localhost.key", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("localhost.crt").unwrap();

    let srv = HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(|| async {
                from_utf8(include_bytes!("data.txt")).unwrap()
            }))
    })
    .disable_signals()
    .bind_openssl("127.0.0.1:8443", builder)?
    .run();

    sys.block_on(srv)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    println!("START SERVER");
    thread::spawn(move || {
        let _ = run_server();
    });

    // Wait for server
    thread::sleep(Duration::from_secs(2));

    let mut x: usize = 1;
    let mut rss = 0;

    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify(SslVerifyMode::NONE);

    let client = ClientBuilder::new()
        .connector(Connector::new().ssl(builder.build()).finish())
        .finish();

    loop {
        let response = client.get("https://localhost:8443/")
            .send()
            .await;

        let _ = match response {
            Ok(mut resp) => resp.body().await,
            Err(e) => {
                println!("Error reading response: {}", e);
                return Ok(())
            }
        };

        let new_rss = stat_self().unwrap().rss;

        if new_rss > rss + 5 {
            println!("{}. {}", x, new_rss);
            rss = new_rss
        }
        x += 1
    }
}
