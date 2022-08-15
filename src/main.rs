
use actix_web::{Responder, HttpResponse, get, HttpServer, App, web::{Data, self}};
use apis::user_apis::{create_user, get_user, get_list_user};
use repository::mongodb_repo::MongoRepo;
mod apis;
mod models;
mod repository;
mod enums;
mod responses;
// mod middlewares;

#[get("/")]
async fn hello() -> impl Responder {
	HttpResponse::Ok().json("Hello from rust and mongoDB")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let db = MongoRepo::init().await;
	let db_data = Data::new(db);
  HttpServer::new(move || 
		App::new()
			.app_data(db_data.clone())
			.service(
				web::scope("/api/v1")
				.service(hello)
				.service(create_user)
				.service(get_user)
				.service(get_list_user)
			)
		)
		.bind(("localhost", 8080))?
		.run()
		.await
}
