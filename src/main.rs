use actix_web::client::Client;
use std::{io, thread, time};
use procinfo::pid::stat_self;

// use jemallocator::Jemalloc;
// #[global_allocator]
// static GLOBAL: Jemalloc = Jemalloc;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let mut x = 1;
    let mut rss = 0;
    loop {
        let _ = Client::default()
            .get("https://google.com")
            // .get("http://google.com")
            .send()
            .await;
        thread::sleep(time::Duration::from_secs(1));
        let new_rss = stat_self().unwrap().rss;
        if new_rss != rss {
            println!("{}. {}", x, new_rss);
            rss = new_rss;
        }
        x += 1;
    };
}
