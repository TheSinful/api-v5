use lazy_static::lazy_static;
use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};
use std::io;
use std::sync::Mutex;

pub mod user;

lazy_static! {
    pub static ref MONGO_CLIENT: Mutex<Option<Client>> = Mutex::new(None);
}

pub async fn connect_to_db() -> mongodb::error::Result<()> {
    let url = "mongodb+srv://app:NUwhtTlxVxWkrXR6@xcel.djriw.mongodb.net/?retryWrites=true&w=majority&appName=Xcel";

    let mut client_options = ClientOptions::parse(url).await?;

    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options)?;
    client
        .database("admin")
        .run_command(doc! {"ping": 1})
        .await?;

    let mut mongo_client = MONGO_CLIENT.lock().unwrap();
    *mongo_client = Some(client);

    Ok(())
}

pub fn get_client() -> Result<Client, io::Error> {
    let mongo_client = MONGO_CLIENT.lock().unwrap();
    match &*mongo_client {
        Some(client) => Ok(client.clone()),
        None => Err(io::Error::new(
            io::ErrorKind::NotFound,
            "MongoDB client is not initialized",
        )),
    }
}
