use crate::standalone::start_standalone_server;

pub mod matchers;
mod net;
pub mod server;
pub mod standalone;

#[tokio::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    print_cli_banner();

    start_standalone_server(3000, false, shutdown_signal())
        .await
        .expect("Error in running server");
}

fn print_cli_banner() {
    log::info!("██████╗███████╗███████╗███████╗███████╗");
    log::info!("██╔═══╝██╔══██║██╔══██║██╔══██║██╔════╝");
    log::info!("██████╗███████║██║  ██║██║  ██║██████╗");
    log::info!(" ╚══██║██╔════╝██║  ██║██║  ██║██╔═══╝");
    log::info!("██████║██║     ███████║███████║██║");
    log::info!(" ╚════╝╚═╝      ╚═════╝ ╚═════╝╚═╝");

    log::info!(
        "Starting {} standalone server V{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
}

#[cfg(not(target_os = "windows"))]
async fn shutdown_signal() {
    let mut hangup_stream = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::hangup())
        .expect("Cannot install SIGINT signal handler");
    let mut sigint_stream =
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())
            .expect("Cannot install SIGINT signal handler");
    let mut sigterm_stream =
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Cannot install SIGINT signal handler");

    tokio::select! {
        _val = hangup_stream.recv() => log::trace!("Received SIGINT"),
        _val = sigint_stream.recv() => log::trace!("Received SIGINT"),
        _val = sigterm_stream.recv() => log::trace!("Received SIGTERM"),
    }
}

#[cfg(target_os = "windows")]
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Cannot install CTRL+C signal handler");
}
