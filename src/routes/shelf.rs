extern crate bodyparser;
extern crate iron;
extern crate persistent;
extern crate postgres;
extern crate serde_json;

use iron::prelude::*;
use iron::status;

use routes::{get_id, get_db};
use routes::item::Item;

include!(concat!(env!("OUT_DIR"), "/shelf.rs"));

impl Shelf {
    pub fn new(row: &postgres::rows::Row) -> Shelf {
        Shelf {
            location_id: row.get("location_id"),
            shelf_id: row.get("shelf_id"),
            name: row.get("name"),
        }
    }
}

pub fn get(req: &mut Request) -> IronResult<Response> {
    let conn = try!(get_db(req));
    let id = try!(get_id(req));

    let rows = conn.query("SELECT * FROM shelf WHERE shelf_id = $1", &[&id]).unwrap();

    let response = match rows.len() {
        0 => Response::with((status::NotFound, format!("Unknown shelf '{}'!", id))),
        1 => {
            let s = serde_json::to_string(&Shelf::new(&rows.get(0))).unwrap();
            Response::with((status::Ok, s))
        }
        // FIXME: return an error message?
        _ => Response::with((status::InternalServerError)),
    };

    Ok(response)
}

pub fn items(req: &mut Request) -> IronResult<Response> {
    let conn = try!(get_db(req));
    let id = try!(get_id(req));

    let rows = conn.query("SELECT * FROM item WHERE shelf_id = $1", &[&id]).unwrap();
    let items = &rows.iter().map(|x| Item::new(&x)).collect::<Vec<_>>();

    Ok(Response::with((status::Ok, serde_json::to_string(items).unwrap())))
}

pub fn create(req: &mut Request) -> IronResult<Response> {
    let conn = try!(get_db(req));

    let body = req.get::<bodyparser::Struct<BaseShelf>>();

    let shelf = match body {
        Ok(Some(shelf)) => shelf,
        Ok(None) => return Ok(Response::with((status::BadRequest, "No shelf specified"))),
        Err(err) => {
            return Err(IronError::new(err,
                                      (status::BadRequest,
                                       "Could not create shelf. Did you use a duplicate name?")))
        }
    };

    let rows = conn.query("INSERT INTO shelf (location_id, name) VALUES ($1, $2) RETURNING *",
                          &[&shelf.location_id, &shelf.name]);

    match rows {
        Ok(r) => {
            let s = serde_json::to_string(&Shelf::new(&r.get(0))).unwrap();
            Ok(Response::with((status::Created, s)))
        }
        Err(err) => {
            let s = err.to_string();
            Err(IronError::new(err, (status::BadRequest, s)))
        }
    }
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    let id = try!(get_id(req));
    let conn = try!(get_db(req));

    let result = conn.execute("DELETE FROM shelf WHERE shelf_id = $1", &[&id]);

    match result {
        Ok(0) => Ok(Response::with(status::NotFound)),
        Ok(_) => Ok(Response::with(status::Ok)),
        Err(err) => Err(IronError::new(err, (status::BadRequest, "Could not delete shelf."))),
    }
}

pub fn update(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::InternalServerError, "Not implemented!")))
}
