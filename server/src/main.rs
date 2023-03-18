use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use log::{info, warn};
use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

#[cfg(test)]
mod app_test;

#[get("/")]
async fn get_handler(
    socket: web::Data<Arc<Mutex<TcpStream>>>,
    info: web::Query<HashMap<String, String>>,
) -> impl Responder {
    let query = serde_urlencoded::to_string(&*info).unwrap();
    let mut socket = socket.lock().unwrap();
    match socket.write(query.as_bytes()) {
        Ok(_) => info!("Sent: {}", query),
        Err(e) => warn!("Error sending data: {}", e),
    };

    HttpResponse::Ok().body("Received GET request")
}

#[post("/")]
async fn post_handler(socket: web::Data<Arc<Mutex<TcpStream>>>, body: String) -> impl Responder {
    let mut socket = socket.lock().unwrap();
    match socket.write(body.as_bytes()) {
        Ok(_) => info!("Sent: {}", body),
        Err(e) => warn!("Error sending data: {}", e),
    };

    HttpResponse::Ok().body(format!("Received POST data: {}", body))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let address = "127.0.0.1:7878";
    let socket = Arc::new(Mutex::new(TcpStream::connect(address).unwrap()));
    info!("Connected to {}", address);

    let http_address = "192.168.50.88:1111";
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&socket)))
            .service(get_handler)
            .service(post_handler)
    })
    .bind(http_address)?
    .run()
    .await
}
