use crate::routes::services::auth::Result;  
use crate::utils::database::get_client;
use crate::utils::database::user::*;
use actix_web::{post, HttpResponse, Responder};

use super::AuthError;


#[post("/login")]
pub async fn login(username: String, password: String) -> impl Responder {
    let login = Login::new(username, password).login().await; 

    match login {
        Ok(_) => HttpResponse::Ok().finish(), 
        Err(e) => HttpResponse::from_error(e)
    }
} 

struct Login {
    username: String, 
    password: String, // unhashed 
}

impl Login 
{
    pub fn new(username: String, password: String) -> Login {
        Login {
            username: username.to_string(), 
            password: password.to_string()
        }
    }

    pub async fn login(&self) -> Result<()> {
        let client = get_client()?;
        let username = self.username.clone(); 
        let password = self.password.clone();  
        let user = User::new(client, username).await?;
        
        if !user.compare_pass(password) {
            return Err(AuthError::InvalidPassword); 
        }

        Ok(())
    }

    


}
