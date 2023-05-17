use crate::db::DBQueryable;

use diesel::pg::PgConnection;
use diesel::prelude::*;

pub mod from;
pub mod cookies;
pub mod apikey;
pub mod api_permissions;

pub trait DBQueryableUtils<T: Queryable<S, diesel::pg::Pg>, S> {
    fn new_by_id(id: i32) -> Self where Self: Sized;
}
