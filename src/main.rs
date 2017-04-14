extern crate iron;
extern crate router;
extern crate dotenv;
#[macro_use] extern crate diesel;

use iron::prelude::*;
use iron::status;

use router::Router;

mod database;


fn main() {
	println!("Start rust-samurai on localhost:3000");

	let connection = database::establish_connection();

	let mut router = Router::new();
	router.get("/", get_index, "get_index");
	router.get("/accounts", get_accounts, "get_accounts");
	router.post("/account/:name", post_account, "post_account");

	Iron::new(router).http("localhost:3000").unwrap();
}

fn get_index(_: &mut Request) -> IronResult<Response> {
	Ok(Response::with((status::Ok, "Welcome to the my homepage!")))
}

fn get_accounts(_: &mut Request) -> IronResult<Response> {
	Ok(Response::with((status::Ok, "List of accounts")))
}

fn post_account(req: &mut Request) -> IronResult<Response> {
	let ref name_opt = req.extensions.get::<Router>().unwrap().find("name");
	match name_opt {
		Some(name) => Ok(Response::with((status::Ok, &format!("create account named {}", name)))),
		None => InternalServerError(Response::with((status::InternalServerError, "name is empty!"))),
	}
}