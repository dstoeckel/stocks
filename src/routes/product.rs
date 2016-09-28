extern crate iron;

extern crate router;
extern crate bodyparser;
extern crate persistent;
extern crate postgres;
extern crate serde;
extern crate serde_json;
extern crate chrono;

use iron::prelude::*;
use iron::status;

use routes::{get_db, get_id};

include!(concat!(env!("OUT_DIR"), "/product.rs"));

impl Product {
    pub fn new(row: &postgres::rows::Row) -> Product {
        return Product {
            product_id: row.get("product_id"),
            name: row.get("name"),
            gtin: row.get("gtin"),
        };
    }
}

impl UnknownProduct {
    fn new(product_id: i32) -> UnknownProduct {
        return UnknownProduct {
            message: "Unknown product specified.",
            product_id: product_id,
        };
    }
}

pub fn get(req: &mut Request) -> IronResult<Response> {
    let conn = try!(get_db(req));
    let id = try!(get_id(req));

    let rows = conn.query("SELECT * FROM product WHERE product_id = $1", &[&id]).unwrap();

    let response = match rows.len() {
        0 => {
            let s = serde_json::to_string(&UnknownProduct::new(id)).unwrap();
            Response::with((status::NotFound, s))
        }
        1 => {
            Response::with((status::Ok,
                            serde_json::to_string(&Product::new(&rows.get(0))).unwrap()))
        }
        // FIXME: return an error message?
        _ => Response::with((status::InternalServerError)),
    };

    Ok(response)
}

pub fn items(req: &mut Request) -> IronResult<Response> {
    let conn = try!(get_db(req));
    let id = try!(get_id(req));

    let rows = conn.query("SELECT * FROM item WHERE product_id = $1", &[&id]).unwrap();
    let items = &rows.iter().map(|x| Product::new(&x)).collect::<Vec<_>>();

    Ok(Response::with((status::Ok, serde_json::to_string(items).unwrap())))
}

pub fn update(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Ho")))
}

pub fn create(req: &mut Request) -> IronResult<Response> {
    let conn = try!(get_db(req));

    let body = req.get::<bodyparser::Struct<BaseProduct>>();

    let product = match body {
        Ok(Some(product)) => product,
        Ok(None) => return Ok(Response::with((status::BadRequest, "No product specified"))),
        Err(err) => {
            let s = err.to_string();
            return Err(IronError::new(err, (status::BadRequest, s)));
        }
    };

    let rows = conn.query("INSERT INTO product (name, gtin) VALUES($1, $2) \
                RETURNING *",
                          &[&product.name, &product.gtin]);

    match rows {
        Ok(row) => {
            Ok(if row.len() == 1 {
                let s = serde_json::to_string(&Product::new(&row.get(0))).unwrap();
                Response::with((status::Created, s))
            } else {
                Response::with((status::InternalServerError))
            })
        }
        Err(err) => {
            Err(IronError::new(err,
                               (status::BadRequest,
                                "Could not create product! Are your location, product, and shelf \
                                 id valid?")))
        }
    }
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    let id = try!(get_id(req));
    let conn = try!(get_db(req));

    let result = conn.execute("DELETE FROM product WHERE product_id = $1", &[&id]);

    match result {
        Ok(0) => Ok(Response::with(status::NotFound)),
        Ok(_) => Ok(Response::with(status::Ok)),
        Err(err) => Err(IronError::new(err, (status::BadRequest, "Could not delete product."))),
    }
}
