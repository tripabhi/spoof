use crate::server::start_server;
use std::future::Future;

pub async fn start_standalone_server<F>(port: u16, expose: bool, shutdown: F) -> Result<(), String>
where
    F: Future<Output = ()> + Send + 'static,
{
    start_server(port, expose, shutdown).await
}
