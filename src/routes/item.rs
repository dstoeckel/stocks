extern crate iron;

extern crate router;
extern crate bodyparser;
extern crate postgres;
extern crate serde;
extern crate serde_json;
extern crate chrono;

use iron::prelude::*;
use iron::status;

use routes::{get_db, get_id};

include!(concat!(env!("OUT_DIR"), "/item.rs"));

impl Item {
    pub fn new(row: &postgres::rows::Row) -> Item {
        return Item {
            item_id: row.get("item_id"),
            product_id: row.get("product_id"),
            location_id: row.get("location_id"),
            shelf_id: row.get("shelf_id"),
            first_added: row.get("first_added"),
            last_moved: row.get("last_moved"),
        };
    }
}

impl UnknownItem {
    fn new(item_id: i32) -> UnknownItem {
        return UnknownItem {
            message: "Unknown item specified.",
            item_id: item_id,
        };
    }
}

pub fn get(req: &mut Request) -> IronResult<Response> {
    let conn = try!(get_db(req));
    let id = try!(get_id(req));

    let rows = conn.query("SELECT * FROM item WHERE item_id = $1", &[&id]).unwrap();

    let response = match rows.len() {
        0 => {
            let s = serde_json::to_string(&UnknownItem::new(id)).unwrap();
            Response::with((status::NotFound, s))
        }
        1 => Response::with((status::Ok, serde_json::to_string(&Item::new(&rows.get(0))).unwrap())),
        // FIXME: return an error message?
        _ => Response::with((status::InternalServerError)),
    };

    Ok(response)
}

pub fn update(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Ho")))
}

pub fn create(req: &mut Request) -> IronResult<Response> {
    let conn = try!(get_db(req));

    let body = req.get::<bodyparser::Struct<BaseItem>>();

    let item = match body {
        Ok(Some(item)) => item,
        Ok(None) => return Ok(Response::with((status::BadRequest, "No item specified"))),
        Err(err) => {
            let s = err.to_string();
            return Err(IronError::new(err, (status::BadRequest, s)));
        }
    };

    let rows = conn.query("INSERT INTO item (product_id, shelf_id, location_id) VALUES($1, $2, $3) \
                RETURNING *",
               &[&item.product_id, &item.shelf_id, &item.location_id]);

    match rows {
        Ok(row) => {
            Ok(if row.len() == 1 {
                let s = serde_json::to_string(&Item::new(&row.get(0))).unwrap();
                Response::with((status::Created, s))
            } else {
                Response::with((status::InternalServerError))
            })
        }
        Err(err) => {
            Err(IronError::new(err,
                               (status::BadRequest,
                                "Could not create item! Are your location, product, and shelf id \
                                 valid?")))
        }
    }
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    let conn = try!(get_db(req));
    let id = try!(get_id(req));

    let result = conn.execute("DELETE FROM item WHERE item_id = $1", &[&id]);

    match result {
        Ok(0) => Ok(Response::with(status::NotFound)),
        Ok(_) => Ok(Response::with(status::Ok)),
        Err(err) => Err(IronError::new(err, (status::BadRequest, "Could not delete item."))),
    }
}
