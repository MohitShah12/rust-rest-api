mod api;
mod models;
mod repository;


#[macro_use]
extern crate rocket;

use api::user_api::{create_user,get_user,update_user,delete_user};
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    println!("Everythin is fine!");
    let db = MongoRepo::init();
    rocket::build()
        .configure(rocket::Config::figment().merge(("port",3000)))
        .manage(db)
        .mount("/",routes![create_user])
        .mount("/", routes![get_user])
        .mount("/", routes![update_user])
        .mount("/", routes![delete_user])

}
