use derive_new::new;

use diesel::r2d2::{ConnectionManager,PooledConnection};
use diesel::PgConnection;

#[derive(new)]
pub struct PgConnect {
    pub connection: PooledConnection<ConnectionManager<PgConnection>>,
}

