use diesel::{prelude::*, result::Error};

use crate::models::User;
use crate::schema::sys_user;
use crate::schema::sys_user::dsl::*;
use supper::PgConnect;



impl PgConnect {
    pub fn find_by_account(&self, searched_account: &str) -> Result<User, Error> {
        let user = sys_user
            .filter(account.eq(searched_account))
            .first::<User>(&*self.connection)?;
        Ok(user)
    }
}
