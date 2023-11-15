use std::{fs::File, io::BufReader, thread::sleep, time::Duration};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use clap::{ArgAction, Parser};
use rand::Rng;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

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

async fn manual_hello() -> impl Responder {
    println!("Receive a GET request on the /api route");
    let mut rng = rand::thread_rng();
    let mut latency: f64 = rng.gen();
    latency = latency * 100.0;
    sleep(Duration::from_millis(latency as u64));

    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let local_ip = "127.0.0.1";
    // let rustls_config = load_rustls_config();

    println!(
        "Launching a simple lagging server, listening on {}:{}",
        local_ip, args.port
    );

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/api", web::get().to(manual_hello))
    })
    .workers(4)
    .bind((local_ip, args.port))?
    // .bind_rustls_021(socket_address, rustls_config)?
    .run()
    .await
}

pub fn load_rustls_config() -> ServerConfig {
    // init server config builder with safe defaults
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open("cert/cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("cert/key.pem").unwrap());

    // convert files to key/cert objects
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    // exit if no keys could be parsed
    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}
