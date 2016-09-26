extern crate iron;
extern crate persistent;
extern crate postgres;
extern crate serde_json;

use iron::prelude::*;
use iron::status;

use routes::{get_id, get_db};
use routes::item::Item;

include!(concat!(env!("OUT_DIR"), "/location.rs"));

impl Location {
    pub fn new(row: &postgres::rows::Row) -> Location {
        Location {
            location_id: row.get("location_id"),
            name: row.get("name"),
        }
    }
}

pub fn get(req: &mut Request) -> IronResult<Response> {
    let conn = try!(get_db(req));
    let id = try!(get_id(req));

    let rows = conn.query("SELECT * FROM location WHERE location_id = $1", &[&id]).unwrap();

    let response = match rows.len() {
        0 => Response::with((status::NotFound, format!("Unknown location '{}'!", id))),
        1 => Response::with((status::Ok, serde_json::to_string(&Location::new(&rows.get(0))).unwrap())),
        // FIXME: return an error message?
        _ => Response::with((status::InternalServerError))
    };

    Ok(response)
}

pub fn list_items(req: &mut Request) -> IronResult<Response> {
    let conn = try!(get_db(req));
    let id = try!(get_id(req));

    let rows = conn.query("SELECT * FROM item WHERE location_id = $1", &[&id]).unwrap();
    let items = &rows.iter().map(|x| {Item::new(&x)}).collect::<Vec<_>>();

    Ok(Response::with((status::Ok, serde_json::to_string(items).unwrap())))
}

pub fn list_shelves(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::InternalServerError, "Not implemented!")))
}

pub fn create(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::InternalServerError, "Not implemented!")))
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::InternalServerError, "Not implemented!")))
}

pub fn update(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::InternalServerError, "Not implemented!")))
}
