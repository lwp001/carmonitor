use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager,Pool};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub fn create_diesel_pool(database_url: impl Into<String>) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create database pool")
}


// use actix::prelude::*;
// use diesel::{
//     prelude::*,
//     r2d2::{ConnectionManager, Pool},
// };
// use failure::Fallible;
// // use log::debug;
// // use webapp::{protocol::model::Session, schema::sessions::dsl::*};

// /// The database executor actor
// pub struct DatabaseExecutor(pub Pool<ConnectionManager<PgConnection>>);

// impl Actor for DatabaseExecutor {
//     type Context = SyncContext<Self>;
// }

// impl DatabaseExecutor {
// 	// add code here
// 	pub fn new(database_url:&str) -> Self {
// 		let manager = ConnectionManager::<PgConnection>::new(database_url);
//         let pool = Pool::builder().build(manager)?;
//         //let db_addr =
//         SyncArbiter::start(num_cpus::get(), move || DatabaseExecutor(pool.clone()))
// 	}
	
// }