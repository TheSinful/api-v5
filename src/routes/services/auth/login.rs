use crate::routes::services::auth::AuthError;
use crate::routes::services::auth::Result;
use crate::utils::database::get_client;
use crate::utils::database::user::*;
use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct ReqBody {
    username: String,
    password: String,
}

#[get("/login")]
pub async fn login(body: web::Json<ReqBody>) -> impl Responder {
    let login = Login::new(body.username.clone(), body.password.clone()).await;
    let login_obj;
    match login {
        Ok(l) => login_obj = l,
        Err(e) => return HttpResponse::from_error(e),
    }

    let login_att = login_obj.login().await;
    let products;
    match login_obj.get_products() {
        Ok(p) => products = p,
        Err(e) => return HttpResponse::from_error(e),
    }

    let body = json!({
        "products": products
    });

    match login_att {
        Ok(_) => HttpResponse::Ok().body(body.to_string()),
        Err(e) => HttpResponse::from_error(e),
    }
}

#[allow(unused)]
struct Login {
    username: String,
    password: String, // unhashed
    user: Option<User>,
}

impl Login {
    pub async fn new(username: String, password: String) -> Result<Login> {
        let client = get_client()?;
        let user = User::find(client, username.clone()).await?;

        Ok(Login {
            username: username.to_string(),
            password: password.to_string(),
            user: Some(user),
        })
    }

    pub async fn login(&self) -> Result<()> {
        let password = self.password.clone();
        let user = self.user.clone().expect("User not found");

        if !user.compare_pass(password) {
            return Err(AuthError::InvalidPassword);
        }

        Ok(())
    }

    pub fn get_products(&self) -> Result<ProductsArray> {
        let products = self.user.clone().expect("User not found").get_products()?;

        Ok(products)
    }
}
