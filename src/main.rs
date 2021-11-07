use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use reqwest::Client;

#[get("/")]
async fn get_temp(client: web::Data<Client>) -> impl Responder {
    let res = client
        .get("https://api.ipify.org/")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    HttpResponse::Ok().body(format!("Hello {}", res))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_temp)
            .app_data(web::Data::new(Client::new()))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
}
