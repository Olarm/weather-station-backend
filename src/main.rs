#[macro_use] extern crate rocket;

use rocket::{
    http::Method,
    serde::json::Json
};
use rocket_cors::{AllowedOrigins, CorsOptions};

use backend::weather::{example, Record};


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/data")]
fn get_data() -> Json<Vec<Record>> {
    let list = example().unwrap();
    Json(list)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    let _rocket = rocket::build()
        .attach(cors.to_cors().unwrap())
        .configure(rocket::Config::figment().merge(("port", 9797)))
        .mount("/", routes![index, get_data])
        .launch()
        .await?;

    Ok(())
}
