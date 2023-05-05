use crate::db::DBQueryable;

use diesel::pg::PgConnection;
use diesel::prelude::*;

pub mod from;
pub mod cookies;
pub mod apikey;
<<<<<<< HEAD
pub mod api_permissions;
=======
>>>>>>> 51d987b925a4c1f27126efdd48feb41281051aa8

pub trait DBQueryableUtils<T: Queryable<S, diesel::pg::Pg>, S> {
    fn new_by_id(id: i32) -> Self where Self: Sized;
}
