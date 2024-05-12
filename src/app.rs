use std::error::Error;
use std::future::Future;
use std::ptr;
use axum::response::Html;
use axum::Router;
use axum::routing::{get, post};
use axum::extract;
use axum::response::IntoResponse;
use axum::http::StatusCode;
use thiserror::Error;
use tokio::signal;
use crate::bootstrap::Args;
use crate::database::Database;
use crate::messaging::{PackType, TrainStationScheduleUpdate};

pub struct App {
    bootstrap_args: Args,
    database: Database
}

#[derive(Error, Debug)]
pub enum AppInitError {
    #[error("database error: {0}")]
    Database(String)
}

impl App {
    pub fn new(bootstrap_args: Args, database: Database) -> Self {
        Self { bootstrap_args, database }
    }
    
    pub async fn run(
        &self,
        shutdown_handler: impl Future<Output = ()> + Send + 'static
    ) -> Result<(), Box<dyn Error>>  {
        // build our application with a route
        let app = Router::new()
            .route("/", get(handler))
            .route("/api/:kind", post(handler_api))
            .fallback(handler_404);

        // run it
        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
            .await?;
        tracing::info!("listening on {}", listener.local_addr().unwrap());
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

static mut APP_PTR: *mut App = ptr::null_mut();

pub async fn init(args: &Args) -> Result<(), AppInitError> {
    let app = init_instance(args).await?;
    // We need to put the data onto heap at first.
    let box_app = Box::new(app);
    // Then leak the boxed App to get the address on heap and record it.
    unsafe {
        APP_PTR = Box::leak(box_app);
    }

    Ok(())
}

async fn init_instance(args: &Args) -> Result<App, AppInitError> {
    let database = Database::load_from_file(args.config().clone()).ok_or(
        AppInitError::Database("Error loading database.".to_string()))?;
    let app = App::new(args.clone(), database);
    Ok(app)
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    app_instance().run(on_app_shutdown()).await
}

pub fn app_instance() -> &'static App {
    unsafe { &*APP_PTR }
}

pub fn app_instance_mut() -> &'static mut App {
    unsafe { &mut *APP_PTR }
}

async fn on_app_shutdown() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)] {
        let sigterm = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("failed to install signal handler")
                .recv()
                .await;
        };
        let sigint = async {
            signal::unix::signal(signal::unix::SignalKind::interrupt())
                .expect("failed to install signal handler")
                .recv()
                .await;
        };
        let sigquit = async {
            signal::unix::signal(signal::unix::SignalKind::quit())
                .expect("failed to install signal handler")
                .recv()
                .await;
        };

        tokio::select! {
            _ = ctrl_c => do_app_shutdown().await,
            _ = sigterm => do_app_shutdown().await,
            _ = sigint => do_app_shutdown().await,
            _ = sigquit => do_app_shutdown().await,
        }
    }

    #[cfg(not(unix))] {
        let terminate = std::future::pending::<()>();

        tokio::select! {
            _ = ctrl_c => do_app_shutdown().await,
            _ = terminate => do_app_shutdown().await,
        }
    }
}

async fn do_app_shutdown() {
    let app = app_instance();
    app.shutdown(app.bootstrap_args.config().clone()).await;
}

pub async fn exit() {
    // Take out and drop the App instance.
    let app;
    unsafe {
        app = Box::from_raw(APP_PTR);
    }
    drop(app);
}
