use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::{InsertOneResult, UpdateResult, DeleteResult},
    sync::{Client,Collection},
};

use crate::models::user_model::User;

pub struct MongoRepo{
    col : Collection<User>,
}

impl MongoRepo{
    pub fn init() -> Self{
        dotenv().ok();
        let uri = match env::var("MONGOURI"){
            Ok(v) => v.to_string(),
            Err(_) => format!("Error in loading env variable"),
        };
        // println!("MongoUri: {}",uri);
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("anotherRustDb");
        let col:Collection<User> = db.collection("User");
        println!("Mongo connected");
        MongoRepo { col }
    }

    pub fn create_user(&self, new_user:User) -> Result<InsertOneResult ,Error>{
        let new_doc = User{
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title
        };

        let user = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating user");
       
        Ok(user)
    }

    pub fn get_user(&self,id:&String) -> Result<User, Error>  {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self.col.find_one(filter,None).ok().expect("Error getting user deatils");
        println!("Hello user:{:?}",user_detail);
        Ok(user_detail.unwrap())
    }

    pub fn update_user(&self , id:&String , new_user:User) -> Result<UpdateResult, Error>{
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id":obj_id};
        let new_doc = doc!{
            "$set":{
                "name":new_user.name,
                "location":new_user.location,
                "title":new_user.title,
            }
        };

        let update_doc = self.col.update_one(filter, new_doc, None).ok().expect("Error while updating user");
        Ok(update_doc)
    }

    pub fn delete_user(&self , id:&String) -> Result<DeleteResult, Error>{
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc!{"_id":obj_id};
        let delete_doc = self.col.delete_one(filter, None).ok().expect("error deleting user");
        Ok(delete_doc)
    }
}
