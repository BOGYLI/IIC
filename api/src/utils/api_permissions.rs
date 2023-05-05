use rocket::request::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use rocket::outcome::IntoOutcome;
use rocket::outcome::try_outcome;
use rocket::http::Cookie;
use rocket::http::CookieJar;
use rocket::http::SameSite;

use crate::db::models::Benutzer;
use crate::utils::DBQueryableUtils;
use crate::db::DBQueryable;


#[derive(Debug)]
pub enum PermissionError {
    Missing,
    Invalid,
}
pub struct ReadPermission();
impl ReadPermission {
	pub fn is_valid(key: Vec<&str>) -> bool {
		// select ApiKey WHERE id = 0
		true
	}
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for ReadPermission {
    type Error = PermissionError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<ReadPermission, PermissionError> {
        let keys: Vec<_> = request.headers().get("x-api-key").collect();
        match keys.len() {
            0 => Outcome::Failure((Status::Unauthorized, PermissionError::Missing)),
            _ if ReadPermission::is_valid(keys) => Outcome::Success(ReadPermission()),
            _ => Outcome::Failure((Status::Unauthorized, PermissionError::Invalid)),
        }
    }
}
pub struct DeletePermission();
impl DeletePermission {
	pub fn is_valid(key: Vec<&str>) -> bool {
		// select ApiKey WHERE id = 1
		true
	}
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for DeletePermission {
    type Error = PermissionError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<DeletePermission, PermissionError> {
        let keys: Vec<_> = request.headers().get("x-api-key").collect();
        match keys.len() {
            0 => Outcome::Failure((Status::Unauthorized, PermissionError::Missing)),
            _ if DeletePermission::is_valid(keys) => Outcome::Success(DeletePermission()),
            _ => Outcome::Failure((Status::Unauthorized, PermissionError::Invalid)),
        }
    }
}
pub struct NewPermission();
impl NewPermission {
	pub fn is_valid(key: Vec<&str>) -> bool {
		// select ApiKey WHERE id = 2
		true
	}
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for NewPermission {
    type Error = PermissionError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<NewPermission, PermissionError> {
        let keys: Vec<_> = request.headers().get("x-api-key").collect();
        println!("{:?}", keys);
        let keys: Vec<_> = vec!["s"];
        match keys.len() {
            0 => Outcome::Failure((Status::Unauthorized, PermissionError::Missing)),
            _ if NewPermission::is_valid(keys) => Outcome::Success(NewPermission()),
            _ => Outcome::Failure((Status::Unauthorized, PermissionError::Invalid)),
        }
    }
}
pub struct UpdatePermission();
impl UpdatePermission {
	pub fn is_valid(key: Vec<&str>) -> bool {
		// select ApiKey WHERE id = 3
		true
	}
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for UpdatePermission {
    type Error = PermissionError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<UpdatePermission, PermissionError> {
        let keys: Vec<_> = request.headers().get("x-api-key").collect();
        match keys.len() {
            0 => Outcome::Failure((Status::Unauthorized, PermissionError::Missing)),
            _ if UpdatePermission::is_valid(keys) => Outcome::Success(UpdatePermission()),
            _ => Outcome::Failure((Status::Unauthorized, PermissionError::Invalid)),
        }
    }
}