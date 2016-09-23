extern crate iron;

extern crate router;
extern crate bodyparser;
extern crate serde;

use iron::prelude::*;
use iron::status;
use router::Router;

include!(concat!(env!("OUT_DIR"), "/user.rs"));

pub fn login(req: &mut Request) -> IronResult<Response> {
    let body = req.get::<bodyparser::Struct<UserCredentials>>();

    match body {
        Ok(Some(creds)) => Ok(Response::with((status::Ok, creds.username))),
        Ok(None) => Ok(Response::with((status::BadRequest, "No body specified"))),
        Err(err) => {
            println!("Error during login: {}!", err);
            Ok(Response::with((status::InternalServerError, "UhOh")))
        }
    }
}

pub fn get(req: &mut Request) -> IronResult<Response> {
    let ref id = req.extensions.get::<Router>().unwrap().find("id").unwrap_or("/");

    Ok(Response::with((status::Ok, *id)))
}

pub fn create(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Ho")))
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Ho")))
}
