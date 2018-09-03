extern crate hyper;
extern crate hyper_tls;

use hyper::rt::{self, Future, Stream};
use hyper::Client;
use hyper_tls::HttpsConnector;
use std::env;
use std::io::{self, Write};

fn main() {
    let url = match env::args().nth(1) {
        Some(url) => url,
        None => {
            println!("Usage: http_getter <url>");
            return;
        }
    };
    let url = url.parse::<hyper::Uri>().unwrap();
    if (url.scheme_part().map(|s| s.as_ref()) != Some("https"))
        && (url.scheme_part().map(|s| s.as_ref()) != Some("http"))
    {
        println!("http_getter only works with 'http/https' URLs.");
        return;
    }
    rt::run(fetch_url(url));
}

fn fetch_url(url: hyper::Uri) -> impl Future<Item = (), Error = ()> {
    let https = HttpsConnector::new(4).expect("TLS initialization failed");
    let client = Client::builder().build::<_, hyper::Body>(https);

    client
        .get(url)
        .and_then(|res| {
            println!("Response: {}", res.status());
            println!("Headers: {:#?}", res.headers());
            res.into_body().for_each(|chunk| {
                io::stdout()
                    .write_all(&chunk)
                    .map_err(|e| panic!("http_getter expects stdout is open, error={}", e))
            })
        }).map(|_| {
            println!();
        }).map_err(|err| {
            eprintln!("Error {}", err);
        })
}
