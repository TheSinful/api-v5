use std::collections::HashMap;
use mongodb::{bson::doc, Client};
use serde::{Deserialize, Serialize};
use crate::utils::password::{verify_password, UnhashedPassword};

#[derive(thiserror::Error, Debug)]
pub enum Error { 
    #[error("Failed to make operation")]
    FailedToMakeOperation(#[from] mongodb::error::Error), 

    #[error("Could not find user")]
    CouldNotFindUser,
}

type UserResult<T> = Result<T, Error>;


pub type Expiration = i32;
pub type Product = String;

/// All available products
pub enum Products {
    FortniteAIOneMonth, 

}

impl Products {
    pub fn to_product(&self) -> Product {
        match *self {
            Self::FortniteAIOneMonth => "fortnite_ai_one_month".to_string()
        } 
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Serials {
    motherboard: String, 
    disk: String, 
    cpu: String
}

/// An actual struct reprsentation of a document in the users collection. 
#[derive(Serialize, Deserialize, Clone)]
pub struct UserDoc {
    username: String,
    password: String, 
    products: HashMap<Product, Expiration>, 
    serials: Serials 
}

async fn find(db_client: &Client, username: &str) -> UserResult<UserDoc>  {
    let db = db_client.database("xcel"); 
    let collection = db.collection::<UserDoc>("users"); 

    let filter = doc! { "username": username.to_owned() }; 
    let find = collection.find_one(filter).await?;         
    
    match find {
        Some(user) => Ok(user), 
        None => Err(Error::CouldNotFindUser)
    }
}

pub struct User {
    client: Client, 
    data: Option<UserDoc>
}

impl User 
{
    pub async fn new(client: Client, username: String) -> UserResult<User> 
    {
        let mut user = User {client: client.clone(), data: None}; 
        let doc = find(&client, &username).await?;
        user.data = Some(doc); 

        Ok(user)
    } 

    pub fn compare_pass(&self, unhashed_password: UnhashedPassword) -> bool {        
        verify_password(&self.data.clone().unwrap().password, &unhashed_password)
    }

    pub fn validate_serials(&self, serials: Serials) -> bool {
        let doc = self.data.clone().unwrap().serials;

        if doc.cpu != serials.cpu || doc.disk != serials.disk || doc.motherboard != serials.motherboard {
            false
        } else {
            true 
        }
    }

}