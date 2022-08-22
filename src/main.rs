
// use std::sync::Mutex;
// use std::sync::Arc;
// use std::thread;

use std::{sync::{Arc, Mutex}, thread};

use actix_web::{Responder, HttpResponse, get, HttpServer, App, web::{Data, self}};
use apis::{user_apis::{init_routes_user}, auth_apis::init_routes_auth};
use repository::mongodb_repo::MongoRepo;
mod apis;
mod models;
mod repository;
mod enums;
mod responses;
mod middlewares;

#[get("/")]
async fn hello() -> impl Responder {
	HttpResponse::Ok().json("Hello from rust and mongoDB")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let myData ="phocode.com";
	let myArcData = Arc::new(Mutex::new(myData)); 
	let myMutex1 = myArcData.clone();
	thread::spawn(move || {
		let lockResult = myMutex1.lock();
		match lockResult {
			Ok(myData) => {
				println!("Locking OK, here is the data: {}", myData)
			},
			Err(error) => {
				println!("Locking failed: {}", error);
			}
		}
	});

	let db = MongoRepo::init().await;
	let db_data = Data::new(db);
  HttpServer::new(move || 
		App::new()
			.app_data(db_data.clone())
			.service(
				web::scope("/api/v1")
				.service(web::scope("/user").configure(init_routes_user))
				.service(web::scope("/auth").configure(init_routes_auth))
			)
		)
		.bind(("localhost", 8080))?
		.run()
		.await
}
