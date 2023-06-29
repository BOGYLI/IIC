use rocket::request::Outcome;
use rocket::http::Status;
use rocket::request::{Request, FromRequest};
//use rocket::request::{self, Request, FromRequest};
use rocket::outcome::IntoOutcome;
use rocket::outcome::try_outcome;
/*use rocket::http::Cookie;
use rocket::http::CookieJar;
use rocket::http::SameSite;*/

use crate::db::models::Benutzer;
use crate::utils::DBQueryableUtils;
use crate::db::DBQueryable;

pub use super::api_permissions::*;

/*#[rocket::async_trait]
impl<'r> FromRequest<'r> for Benutzer {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        request.cookies()
            .get_private("user_id")
            .and_then(|c| c.value().parse().ok())
            .map(|id| Benutzer::new_by_id(id).get(&mut crate::db::v1::establish_connection()).ok().unwrap())
            .or_forward(())
    }
}*/


#[rocket::async_trait]
impl<'r> FromRequest<'r> for Benutzer {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Benutzer, ()> {
        request.cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .and_then(|id| Benutzer::new_by_id(id).get(&mut crate::db::establish_connection()).ok())
            .or_forward(())
    }
}



pub struct Admin{
    pub user: Benutzer
}
impl Benutzer{
	pub fn is_admin(&self) -> bool {
		//return self.rolle == "Admin"
		true
	}
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Admin {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Admin, ()> {
        // This will unconditionally query the database!
        let user = try_outcome!(request.guard::<Benutzer>().await);
        if user.is_admin() {
            Outcome::Success(Admin { user })
        } else {
            Outcome::Forward(())
        }
    }
}



pub struct Lehrer{
    pub user: Benutzer
}
impl Benutzer{
	pub fn is_lehrer(&self) -> bool {
	    //return self.rolle == "Lehrer"
	    true
	}
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Lehrer {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Lehrer, ()> {
        // This will unconditionally query the database!
        let user = try_outcome!(request.guard::<Benutzer>().await);
        if user.is_lehrer() {
            Outcome::Success(Lehrer { user })
        } else {
            Outcome::Forward(())
        }
    }
}



/*
#[derive(Debug)]
pub enum ApiKeyError {
    BadCount,
    Missing,
    Invalid,
}
pub struct ApiKey(String);
impl ApiKey {
	pub fn is_valid(key: &str, uri: &rocket::http::uri::Origin<'_>) -> bool {
		//let count = (*uri).segment_count();
		//let segments = *uri.segments();
		
		/*match segments[0].to_string() {
			"api" => {},
			_ => {}
		}*/
		true
	}
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ApiKeyError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<ApiKey, ApiKeyError> {
        let keys: Vec<_> = request.headers().get("x-api-key").collect();
        let uri = request.uri();
        println!("{}", uri);
        match keys.len() {
            0 => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            1 if ApiKey::is_valid(keys[0], uri) => Outcome::Success(ApiKey(keys[0].to_string())),
            1 => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
            _ => Outcome::Failure((Status::BadRequest, ApiKeyError::BadCount)),
        }
    }
}*/



/*
*************************************************************************************************************************************
*************************************************************************************************************************************
*/
/*
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
}*/
/*
*************************************************************************************************************************************
*************************************************************************************************************************************
*/


pub struct HTMLPermission();
impl HTMLPermission {
	/*pub fn is_valid(key: &str) -> bool {
		// select ApiKey WHERE id = 4
		true
	}*/ //private Cookie -> simple 1 / 0 Unterscheidung
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for HTMLPermission {    
    type Error = PermissionError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<HTMLPermission, PermissionError> {
        match request.cookies()
            .get_private("html_access")
            .and_then(|cookie| if cookie.value() == "1"  {println!("{}", cookie.value()); Some(HTMLPermission())} else {println!("{}", cookie.value()); None}) {
		Some(permission) => Outcome::Success(permission),
		None => Outcome::Failure((Status::Locked, PermissionError::Invalid)),
	}
    }
}




/*
// Authentizierung als irgendein angemeldeter Benutzer mit Rolle
#[derive(Serialize)]
pub struct Rolle(Benutzer);
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Rolle {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        request.cookies()
            .get_private("user_id")
            .and_then(|c| c.value().parse().ok())
            .map(|id| Benutzer::new_by_id(id).get(&mut crate::db::v1::establish_connection()).ok().expect("Database error"))
            .map(|benutzer| Rolle(benutzer))
            .or_forward(())
    }
}


// Authentizierung als irgendein Lehrer
pub struct Lehrer();
#[rocket::async_trait]
impl Lehrer {
	pub fn from(rolle: String) -> Option<Self> {
		if rolle == "Lehrer" {
			Some(Lehrer())
		} else {
			None
		}
	}
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Lehrer {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        request.cookies()
            .get_private("user_id")
            .and_then(|c| c.value().parse().ok())
            .map(|id| Benutzer::new_by_id(id).get(&mut crate::db::v1::establish_connection()).ok().unwrap())
            .map(|benutzer| Lehrer::from(benutzer.rolle).or())
            .or_forward(())
    }
}


// Authentizierung als irgendein Schueler
pub struct Schueler(Benutzer);
impl Benutzer {
	pub fn is_schueler(&self) -> bool {
		self.rolle == "Schueler"
	}
}
impl<'a> FromRequest<'a> for Schueler {
    type Error = ();

    fn from_request(request: &'a Request<'_>) -> request::Outcome<Schueler, ()> {
        // This will unconditionally query the database!
        let user = request.guard::<Benutzer>().unwrap();

        if user.is_schueler() {
            Outcome::Success(Schueler(user))
        } else {
            Outcome::Forward(())
        }
    }
}
*/
