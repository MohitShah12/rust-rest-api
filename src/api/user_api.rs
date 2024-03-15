use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
use mongodb::{bson::oid::ObjectId, results::{InsertOneResult, DeleteResult}};
use rocket::{http::Status, serde::json::Json, State};
use pwhash::bcrypt;

#[post("/user",data="<new_user>")]
pub fn create_user(db:&State<MongoRepo>, new_user:Json<User>,) -> Result<Json<InsertOneResult>,Status>{

    let hashPass = bcrypt::hash(new_user.location.to_owned()).unwrap();

    println!("{}", hashPass);

    let data = User{
        id:None,
        name:new_user.name.to_owned(),
        location:hashPass.to_owned(),
        title:new_user.title.to_owned(),
    };
    let user_detail = db.create_user(data);
    match user_detail{
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/user/<path>")]
pub fn get_user(db:&State<MongoRepo>, path:String) -> Result<Json<User>,Status>{
    let id = path;
    if id.is_empty(){
        return  Err(Status::BadRequest);
    };
    let user_detail = db.get_user(&id);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::NotFound),
    }
}

#[put("/user/<path>", data="<new_user>")]
pub fn update_user(db:&State<MongoRepo>, path:String, new_user:Json<User>) -> Result<Json<User>, Status>{
    let id = path;
    if id.is_empty(){
        return Err(Status::BadRequest);
    };
    let data = User{
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name:new_user.name.to_owned(),
        location:new_user.location.to_owned(), 
        title:new_user.title.to_owned(),
    };
    let update_result = db.update_user(&id, data);
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let update_user_info = db.get_user(&id);
                return match update_user_info {
                   Ok(user) => Ok(Json(user)),
                   Err(_) => Err(Status::InternalServerError)
                };
            }
            else{
                return  Err(Status::InternalServerError);
            }
        }
        Err(_) => Err(Status::InternalServerError)
    }
}

#[delete("/user/<path>")]
pub fn delete_user(db:&State<MongoRepo>,path:String) -> Result<Json<DeleteResult>, Status>{
    let id = path;
    if id.is_empty(){
        return Err(Status::BadRequest);
    };
    let delete_result = db.delete_user(&id);
    match delete_result {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError)
    }
}