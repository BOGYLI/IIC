pub use diesel_ext_insertable_proc_macro::DBInsertable;

pub trait DBInsertable<T: diesel::Queryable<S, diesel::pg::Pg>, S> {
    #[allow(clippy::new_ret_no_self)]
    fn new(self, conn: &mut diesel::PgConnection) -> Result<T, diesel::result::Error> where Self: Sized;
}

pub use diesel_ext_queryable_proc_macro::DBQueryable;

pub trait DBQueryable<T: diesel::Queryable<S, diesel::pg::Pg>, S> {
    fn get_all(conn: &mut diesel::PgConnection) -> Result<Vec<Self>, diesel::result::Error> where Self: Sized;
    fn get(&self, conn: &mut diesel::PgConnection) -> Result<Self, diesel::result::Error> where Self: Sized;
    fn delete(&self, conn: &mut diesel::PgConnection) -> Result<usize, diesel::result::Error>; // evtl. ohne 'bool'
    fn update(&self, conn: &mut diesel::PgConnection) -> Result<usize, diesel::result::Error>;
}

pub use diesel_ext_checkable_proc_macro::DBCheckable;

pub trait DBCheckable<T: diesel::Queryable<S, diesel::pg::Pg>, S> {
    fn check(&self, conn: &mut diesel::PgConnection) -> Result<Self, diesel::result::Error> where Self: Sized;
}

pub use diesel_ext_connection_proc_macro::connection;