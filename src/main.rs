mod data;
mod messaging;
mod database;
mod app;
mod constants;

use std::error::Error;
use std::ptr;
use tokio::signal;

use app::App;

static mut APP_PTR: *const App = ptr::null();

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = app::init_app("run/config.json".to_string()).await?;
    unsafe {
        APP_PTR = &app;
    }
    app.run(on_app_shutdown()).await
}

async fn on_app_shutdown() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
        let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => do_app_shutdown().await,
        _ = terminate => do_app_shutdown().await,
    }
}

async fn do_app_shutdown() {
    unsafe {
        (*APP_PTR).shutdown("run/config.json".to_string()).await
    }
}
