extern crate iron;

extern crate router;
extern crate bodyparser;
extern crate serde;

use iron::prelude::*;
use iron::status;
use router::Router;

// include!(concat!(env!("OUT_DIR"), "/user.rs"));

pub fn get(req: &mut Request) -> IronResult<Response> {
    let ref id = req.extensions.get::<Router>().unwrap().find("id").unwrap_or("/");
    let db = req.extensions.get::<StocksDatabase>().unwrap();

    Ok(Response::with((status::Ok, *id)))
}

pub fn update(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Ho")))
}

pub fn create(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Ho")))
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Ho")))
}
