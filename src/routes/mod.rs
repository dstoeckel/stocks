extern crate iron;
extern crate persistent;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate router;

use iron::prelude::*;
use iron::status;
use persistent::Read;
use router::Router;
use database::{Connection, StocksDatabase};

pub mod user;
pub mod item;
pub mod location;

pub fn get_id(req: &Request) -> Result<i32, IronError> {
    let router = req.extensions.get::<Router>().unwrap();
    let id_param = router.find("id").unwrap();

    match id_param.parse::<i32>() {
        Ok(i) => Ok(i),
        Err(e) => Err(IronError::new(e, status::BadRequest)),
    }
}

pub fn get_db(req: &mut Request) -> Result<Connection, IronError> {
    let db = req.get::<Read<StocksDatabase>>().unwrap();

    match db.get() {
        Ok(conn) => Ok(conn),
        Err(err) => Err(IronError::new(err, status::InternalServerError)),
    }
}

pub fn setup(router: &mut Router) {
    // Items
    router.get("/item/:id", item::get, "item_get");
    router.delete("/item/:id", item::delete, "item_delete");
    router.post("/item", item::create, "item_create");
    router.put("/item/:id", item::update, "item_update");

    // Locations
    router.get("/location/:id", location::get, "loc_get");
    router.get("/location/:id/items", location::items, "loc_items");
    router.get("/location/:id/shelves", location::shelves, "loc_shelves");
    router.delete("/location/:id", location::delete, "loc_delete");
    router.post("/location", location::create, "loc_create");
    router.put("/location/:id", location::update, "loc_update");

    // Users
    router.post("/user/login", user::login, "user_login");
    router.post("/user", user::create, "user_create");
    router.get("/user/:id", user::get, "user_get");
    router.delete("/user/:id", user::delete, "user_delete");
}
