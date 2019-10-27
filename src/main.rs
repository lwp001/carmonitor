#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::style,
    clippy::complexity,
    clippy::restriction,
    clippy::perf,
    clippy::cargo,
    clippy::correctness
)]
#![allow(
    clippy::module_name_repetitions,
    clippy::wildcard_enum_match_arm,
    clippy::implicit_return,
    clippy::multiple_crate_versions,
    clippy::missing_docs_in_private_items,
    clippy::too_many_arguments,
    clippy::multiple_inherent_impl
)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;


mod config;
mod errors;
mod models;
mod repositories;
mod schema;
mod services;
mod users;
mod utils;


use std::sync::Arc;


use actix_cors::Cors;
use actix_files::Files;
use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use log::info;
use actix_service::Service;
use futures::future::Future;
// use snowflake::ProcessUniqueId;

use crate::config::Config;
use crate::utils::create_diesel_pool;


use crate::users::{login_service, me_service};
// use sys_manager::{org_tree_service,org_add_service,org_list_service};

embed_migrations!("./migrations");


fn main() -> std::io::Result<()> {
    dotenv().ok();
    // ::std::env::set_var("RUST_LOG", "actix_web=info");
    // ::std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let config = Config::new().expect("Invalid Config");
    let address = config.address();
    let assets = config.assets();
    info!("Server at {} and assets = {}", &address, &assets);

    let pool = create_diesel_pool(&config.database_url);
    // let connection = pool.clone().get().unwrap();
    // embedded_migrations::run(&*connection).expect("Embed migrations failed");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(config.clone())
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.1.0"))
            .wrap(middleware::Compress::default())
            // .wrap(middleware::Logger::default())
            .wrap_fn(|req, srv| {
                // println!("You requested: {},{:?}", req.path(),req.headers());
                println!("You requested: {}", req.path());
                srv.call(req).map(|res| {
                    println!(" response：{:?}",res);
                    res
                })
            })
            .service(
                web::scope("/api")
                    .wrap(Cors::default())
                    .service(
                        web::resource("/user/login").route(web::post().to_async(login_service)),
                    )
                    .service(web::resource("/user/info").route(web::get().to(me_service))),
                    // .service(web::resource("/organization/tree/{id}").route(web::get().to_async(org_tree_service)))
                    // .service(web::resource("/organization/add").route(web::post().to_async(org_add_service))) 
                    // .service(web::resource("/organization/list").route(web::get().to_async(org_list_service))) 
                    // .service(
                    //     web::resource("/org/sources")
                    //         .route(web::get().to_async(my_rss_sources_service))
                    //         .route(web::post().to_async(add_rss_source_service)),
                    // ),
            )
            .service(Files::new("/", &assets).index_file("index.html"))
    })
    .bind(&address)?
    .run()
}
