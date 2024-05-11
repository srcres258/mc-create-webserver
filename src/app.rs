use std::error::Error;
use std::future::Future;
use axum::response::Html;
use axum::Router;
use axum::routing::{get, post};
use axum::extract;
use axum::response::IntoResponse;
use axum::http::StatusCode;
use thiserror::Error;
use crate::database::Database;
use crate::messaging::{PackType, TrainStationScheduleUpdate};

pub struct App {
    database: Database
}

#[derive(Error, Debug)]
pub enum AppInitError {
    #[error("database error: {0}")]
    Database(String)
}

impl App {
    pub fn new(database: Database) -> Self {
        Self { database }
    }
    
    pub async fn run(
        &self,
        shutdown_handler: impl Future<Output = ()> + Send + 'static
    ) -> Result<(), Box<dyn Error>>  {
        println!("Hello, world!");

        // build our application with a route
        let app = Router::new()
            .route("/", get(handler))
            .route("/api/:kind", post(handler_api))
            .fallback(handler_404);

        // run it
        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
            .await?;
        println!("listening on {}", listener.local_addr().unwrap());
        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_handler)
            .await?;

        Ok(())
    }

    pub async fn shutdown(&self, database_path: String) {
        self.save_database(database_path.clone()).await
    }

    async fn save_database(&self, database_path: String) {
        self.database.save_to_file(database_path).unwrap()
    }
}

pub async fn init_app(config_file_path: String) -> Result<App, AppInitError> {
    let database = Database::load_from_file(config_file_path).ok_or(
        AppInitError::Database("Error loading database.".to_string()))?;
    let app = App::new(database);
    Ok(app)
}

async fn handler() -> Html<String> {
    use rtml::*;

    // Use the macros to generate some HTML
    let document: String = html! {
        .lang = "en",
            head!{
                title!{
                    "Title of the document"
                }
            },
            body!{
                    div!{
                        "text  测试",
                        h1!{
                            "This is a heading"
                        },
                        p!{
                            "This is a paragraph"
                        }
                    },
                    table!{
                        tr!{
                            td!["Cell 1,1"],
                            td!["Cell 1,2"]
                        },
                        tr!{
                            td!["Cell 2,1"],
                            td!["Cell 2,2"]
                        }
                    }
            }
    }.render();
    println!("{}", document);

    Html(document)
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Your request has not been satisfied yet.")
}

async fn handler_api(
    extract::Path(kind): extract::Path<String>,
    data: String
) -> impl IntoResponse {
    println!("Received API {}: {}", kind, data);
    let kind_type = PackType::of(kind.as_str());
    match kind_type {
        Some(PackType::TrainStationScheduleUpdate) => {
            let schedule: Result<TrainStationScheduleUpdate, _> = serde_json::from_str(data.as_str());
            match schedule {
                Ok(schedule) => {
                    //todo
                    (StatusCode::OK, "OK")
                }
                Err(_) => (StatusCode::BAD_REQUEST, "Invalid JSON for API TrainStationScheduleUpdate.")
            }
        }
        None => (StatusCode::BAD_REQUEST, "The request API kind is invalid.")
    }
}

