use actix_web::{HttpResponse, ResponseError};
use crate::utils::database::user::Error as OperationError; 

pub mod login; 

pub type Result<T> = std::result::Result<T, AuthError>; 

#[derive(thiserror::Error, Debug)]
pub enum AuthError {

    #[error("Couldn't find user.")] 
    CouldntFindUser, 

    #[error("Invalid password")]
    InvalidPassword, 

    #[error("Could not get client")]
    CouldNotGetClient(#[from] std::io::Error), 

    #[error("User operation error")]
    UserOperationError(#[from] OperationError)
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AuthError::CouldntFindUser => HttpResponse::Unauthorized().body("Invalid username or password."), 
            AuthError::CouldNotGetClient(_) => HttpResponse::InternalServerError().body("Internal server error."), 
            AuthError::InvalidPassword => HttpResponse::Unauthorized().body("Invalid username or password."), 
            AuthError::UserOperationError(e) => {
                match e {
                    OperationError::CouldNotFindUser => HttpResponse::Unauthorized().body("Invalid username or password."),  
                    OperationError::FailedToMakeOperation(_) => HttpResponse::InternalServerError().body("Internal server error.")
                }
            }
        }
    }
}