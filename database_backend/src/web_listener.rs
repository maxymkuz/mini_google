use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

use crate::database;

#[derive(Debug, Serialize, Deserialize)]
struct SearchQuery {
    text: String,
}

/// The search callback thingy
async fn search(
    client: web::Data<database::Elasticsearch>,
    query: web::Json<SearchQuery>,
) -> HttpResponse {
    // Try to query the database, if everything goes well, return the json, if not, return 409
    let search_result = match crate::get_response(&client, &query.text).await {
        Ok(x) => x,
        Err(_) => return HttpResponse::Conflict().finish(),
    };
    println!("Response: {:?}", &search_result);
    HttpResponse::Ok().json(search_result)
}

/// Launches the web server that listens for crawlers inserts and web backend queries
pub async fn launch_server() -> std::io::Result<()> {
    // Set up logger
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Launch the server
    HttpServer::new(move || {
        let client = database::Elasticsearch::default();
        App::new()
            .data(client.clone())
            .wrap(middleware::Logger::default())
            // TODO: We do need to limit data. But not now cuz the jsons are going to be cut off.
            // We need to figure out a way for Elasticsearch to give us less data back (first 100
            // characters or whatever instead of the full text.
            //.data(web::JsonConfig::default().limit(4096))
            .service(web::resource("/search").route(web::post().to(search)))
    })
    .bind("127.0.0.1:8020")?
    .run()
    .await
}
