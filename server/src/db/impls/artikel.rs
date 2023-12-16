use diesel::PgConnection;

use crate::db::models::*;



impl Parameter {
    pub fn get_connected_with(templateId: i32, conn: &mut PgConnection) -> Result<Vec<Parameter>, diesel::result::Error> {
        todo!();
    }
}