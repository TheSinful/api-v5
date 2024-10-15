use crate::utils::password::{hash_password, verify_password, HashedPassword, UnhashedPassword};
use mongodb::{bson::doc, Client};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to make operation")]
    FailedToMakeOperation(#[from] mongodb::error::Error),

    #[error("Could not find user")]
    CouldNotFindUser,

    #[error("Username already exists.")]
    UsernameAlreadyExists,
}

pub type UserResult<T> = Result<T, Error>;
pub type Expiration = i32;
pub type Product = String;
pub type ProductsArray = Option<HashMap<Product, Expiration>>;

/// All available products
#[allow(unused)]
pub enum Products {
    FortniteAIOneMonth,
}

impl Products {
    #[allow(unused)]
    pub fn to_product(&self) -> Product {
        match *self {
            Self::FortniteAIOneMonth => "fortnite_ai_one_month".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Serials {
    motherboard: String,
    disk: String,
    cpu: String,
}

/// An actual struct reprsentation of a document in the users collection.
#[derive(Serialize, Deserialize, Clone)]
struct UserDoc {
    username: String,
    password: String,
    pub products: ProductsArray,
    serials: Option<Serials>, // someone may register but serials are upated when they actually launch the client
}

impl UserDoc {
    pub fn new(
        username: String,
        password: String,
        products: ProductsArray,
        serials: Option<Serials>,
    ) -> UserDoc {
        UserDoc {
            username: username,
            password: password,
            products: products,
            serials: serials,
        }
    }
}

async fn find(db_client: &Client, username: &str) -> UserResult<UserDoc> {
    let db = db_client.database("xcel");
    let collection = db.collection::<UserDoc>("users");

    let filter = doc! { "username": username.to_owned() };
    let find = collection.find_one(filter).await?;

    match find {
        Some(user) => Ok(user),
        None => Err(Error::CouldNotFindUser),
    }
}

async fn create(
    db_client: &Client,
    username: String,
    password: UnhashedPassword,
    serials: Option<Serials>,
) -> UserResult<UserDoc> {
    let db = db_client.database("xcel");
    let collection = db.collection::<UserDoc>("users");
    let user = UserDoc::new(username, hash_password(password), None, serials);
    collection.insert_one(&user).await?;

    Ok(user)
}

async fn exists(db_client: &Client, username: String) -> UserResult<bool> {
    let db = db_client.database("xcel");
    let collection = db.collection::<UserDoc>("users");

    let filter = doc! { "username": &username };
    let user_doc = collection.find_one(filter).await?;

    Ok(user_doc.is_some())
}

#[derive(Clone)]
pub struct User {
    data: Option<UserDoc>,
}

impl User {
    pub async fn find(client: Client, username: String) -> UserResult<User> {
        let mut user = User { data: None };
        let doc = find(&client, &username).await?;
        user.data = Some(doc);

        Ok(user)
    }

    pub async fn new(
        client: Client,
        username: String,
        password: HashedPassword,
        serials: Option<Serials>,
    ) -> UserResult<User> {
        if exists(&client, username.clone()).await? {
            return Err(Error::UsernameAlreadyExists);
        }

        let mut user = User { data: None };
        let doc = create(&client, username, password, serials).await?;
        user.data = Some(doc);

        Ok(user)
    }

    pub fn compare_pass(&self, unhashed_password: UnhashedPassword) -> bool {
        verify_password(&self.data.clone().unwrap().password, &unhashed_password)
    }

    #[allow(unused)]
    pub fn validate_serials(&self, serials: Serials) -> bool {
        let doc = self.data.clone().unwrap().serials;

        let doc_serials;
        match doc {
            Some(s) => doc_serials = s,
            None => return false,
        }

        if doc_serials.cpu != serials.cpu
            || doc_serials.disk != serials.disk
            || doc_serials.motherboard != serials.motherboard
        {
            false
        } else {
            true
        }
    }

    pub fn get_products(&self) -> UserResult<ProductsArray> {
        Ok(self.data.clone().unwrap().products)
    }
}
