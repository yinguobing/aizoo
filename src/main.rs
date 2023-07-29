use serde::Serialize;
use warp::{reply::json, Filter, Rejection, Reply};

type WebResult<T> = std::result::Result<T, Rejection>;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

pub async fn health_checker_handler() -> WebResult<impl Reply> {
    const MESSAGE: &str = "Build simple API with RUST";

    let response_json = &GenericResponse {
        status: "Sucess".to_string(),
        message: MESSAGE.to_string(),
    };

    Ok(json(response_json))
}

#[tokio::main]
async fn main() {
    let health_checker = warp::path!("api" / "healthchecker")
        .and(warp::get())
        .and_then(health_checker_handler);
    let routes = health_checker.with(warp::log("api"));
    println!("Server started.");
    warp::serve(routes).run(([0, 0, 0, 0], 80)).await;
}
