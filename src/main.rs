use actix_web::{get, web, App, HttpResponse, HttpServer};
use portpicker::{is_free, pick_unused_port};
use serde::Deserialize;
use cached::proc_macro::cached;

mod amort;

#[derive(Debug, Deserialize)]
struct AmortizeRequest {
    loan_amount: f32,
    terms_in_months: i16,
    annual_interest_rate: f32,
}

#[get("/")]
async fn amortize(info: web::Query<AmortizeRequest>) -> HttpResponse {
    let table = amort::amortize(
        info.loan_amount,
        info.terms_in_months,
        info.annual_interest_rate,
    );

    HttpResponse::Ok()
        .content_type("application/json")
        .json(table)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16;

    if is_free(8080) {
        port = 8080
    } else {
        port = pick_unused_port().expect("No ports free");
    }

    println!("Server Running at 127.0.0.1:{}", port);

    let mut address = String::from("127.0.0.1");
    
    address.push_str(":");
    address.push_str(&port.to_string());

    HttpServer::new(|| App::new().service(amortize))
        .bind(address)?
        .run()
        .await
}
