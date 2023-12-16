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