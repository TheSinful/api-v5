use crate::routes::services::auth::Result;
use crate::utils::database::get_client;
use crate::utils::database::user::Serials;
use crate::utils::database::user::User;
use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ReqBody {
    username: String,
    password: String,
    serials: Option<Serials>,
}

#[post("/register")]
pub async fn register(body: web::Json<ReqBody>) -> impl Responder {
    let username = body.username.clone();
    let password = body.password.clone();
    let serials = body.serials.clone();

    let register = Register::new(username, password, serials).await;

    match register {
        Ok(_) => {}
        Err(e) => {
            return HttpResponse::from_error(e);
        }
    }

    HttpResponse::Ok().into()
}

struct Register;

impl Register {
    pub async fn new(username: String, password: String, serials: Option<Serials>) -> Result<()> {
        let client = get_client()?;
        User::new(client, username, password, serials).await?;

        Ok(())
    }
}
