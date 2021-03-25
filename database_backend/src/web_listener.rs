use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use crate::database;

#[derive(Debug, Serialize, Deserialize)]
struct SearchQuery {
    query: String,
}

#[get("/search")]
async fn search(
    client: web::Data<&database::Elasticsearch>,
    query: web::Json<SearchQuery>,
) -> HttpResponse {
    // Try to query the database, if everything goes well, return the json, if not, return 409
    let search_result = match database::get_search(
        &client,
        serde_json::json!({"query": {"match": {"full_text": query.query }}}),
    )
    .await
    {
        Ok(x) => x,
        Err(_) => return HttpResponse::Conflict().finish(),
    };
    HttpResponse::Ok().json(search_result)
}

#[actix_web::main]
pub async fn launch_server() -> std::io::Result<()> {
    // Set up logger
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Launch the server
    // TODO: I HAVE NO FUCKING IDEA WHY THIS FUKCING SHIT DOESNT COMPILE I LITERALLY ALMOST COPIED
    // THIS FROM THE EXAMPLES I HATE THIS SHIT I DO NOT HAVE ANY MORE FUKCING PATIENCE FOR THESE
    // STUPID LIBRARIES I HATE ELASTICSEARCH I HATE ACTIX I HATE ROCKET WHY CAN'T YOU JUST BE
    // FUCKINFG CONSISTENT AND LOGICAL AND I DON'T KNOW LIKE FUCKING PROVIDE VALID EXAMPLES YOU
    // FUCKING BITCHES . WHAT DOES TRAIT FACTORY<_____> NOT IMPLEMENTED MY ASS FUCKING MEAN WTF IS
    // THIS FUCKING LOAD OF FUCKING SHIT.
    //
    // If you have any deeper understanding of why this problem might occur plz help me out I've
    // been staring at ElasticSearch documentatin all day and my brain is non-existent.
    HttpServer::new(move || {
        let client = database::Elasticsearch::default();
        App::new()
            .data(&client)
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(web::resource("/search").route(web::post().to(search)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
