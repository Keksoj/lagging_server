use std::{fs::File, io::BufReader, thread::sleep, time::Duration};

use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use rand::Rng;
use rustls::ServerConfig;
use rustls_pemfile::{certs, private_key};

#[derive(Parser, PartialEq, Eq, Clone, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 'p', long = "port")]
    port: u16,
}

#[get("/")]
async fn hello() -> impl Responder {
    println!("Received a request, waiting…");
    for i in 0..10 {
        match i {
            0 => println!("{} second", i),
            _ => println!("{} seconds", i),
        }
        sleep(Duration::from_secs(1));
    }
    println!("Responded!");
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/latency")]
async fn latency() -> impl Responder {
    println!("Receive a GET request on the /latency route");
    let mut rng = rand::thread_rng();
    let mut latency: f64 = rng.gen();
    latency = latency * 100.0;
    sleep(Duration::from_millis(latency as u64));

    HttpResponse::Ok().body(format!("Hi, I come with {} milliseconds latency", latency))
}

#[get("/api")]
async fn api() -> impl Responder {
    println!("Receive a GET request on the /api route");

    HttpResponse::Ok().body("Hey There!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let local_ip = "0.0.0.0";
    // let rustls_config = load_rustls_config();

    println!(
        "Launching a simple lagging server, listening on {}:{}",
        local_ip, args.port
    );

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(latency)
            .service(api)
    })
    .workers(4)
    .bind((local_ip, args.port))?
    // .bind_rustls_021(socket_address, rustls_config)?
    .run()
    .await
}

pub fn load_rustls_config() -> ServerConfig {
    // init server config builder with safe defaults
    let config = ServerConfig::builder().with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open("cert/cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("cert/key.pem").unwrap());

    // convert files to key/cert objects
    let cert_chain = certs(cert_file)
        .into_iter()
        .map(|c| c.expect("could not create cert from file"))
        .collect();

    let private_key = private_key(key_file)
        .expect("could not create key from file")
        .unwrap_or_else(|| panic!("no key in file"));

    config.with_single_cert(cert_chain, private_key).unwrap()
}
