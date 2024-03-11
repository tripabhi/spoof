use bytes::Bytes;

use http_body_util::Full;
use hyper::body::Incoming;
use hyper::service::service_fn;
use hyper::{Error, Request, Response};
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto::Builder;
use std::future::Future;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use futures_util::{pin_mut, FutureExt};
use tokio::net::{TcpListener, TcpStream};

pub(crate) async fn start_server<F>(port: u16, expose: bool, shutdown: F) -> Result<(), String>
where
    F: Future<Output = ()> + Send + 'static,
{
    let host: [u8; 4] = if expose { [0, 0, 0, 0] } else { [127, 0, 0, 1] };

    let addr: SocketAddr = SocketAddr::from((host, port));

    let listener = TcpListener::bind(addr)
        .await
        .expect(&*format!("Failed to bind to {}", addr.to_string()));

    log::info!("Listening on {}", addr.to_string());

    let service =
        service_fn(move |req| async move { Ok::<_, Error>(handle_server_request(req).await) });

    // Channel to send shutdown signal
    let (signal_tx, signal_rx) = tokio::sync::watch::channel(());
    let signal_tx = Arc::new(signal_tx);

    // Launches task to wait for Shutdown signal and then drop the signal receiver
    tokio::spawn(async move {
        shutdown.await;
        log::trace!("Received graceful shutdown signal. Signalling threads to stop");
        drop(signal_rx);
    });

    // Channel to know if all connections are closed/gracefully shutdown
    let (close_tx, close_rx) = tokio::sync::watch::channel(());

    loop {
        let (tcp_stream, socket_addr) = tokio::select! {
            conn = tcp_accept(&listener) => {
                match conn {
                    Some(conn) => conn,
                    None => continue,
                }
            }

            _ = signal_tx.closed() => {
                log::trace!("Shutdown Signal received, not accepting any new connections");
                break;
            }
        };

        let tcp_stream = TokioIo::new(tcp_stream);

        log::trace!("connection to tcp socket : {socket_addr} is established");

        let signal_tx = Arc::clone(&signal_tx);
        let close_rx = close_rx.clone();

        tokio::spawn(async move {
            let builder = Builder::new(TokioExecutor::new());
            let conn = builder.serve_connection_with_upgrades(tcp_stream, service);

            pin_mut!(conn);

            let signal_closed = signal_tx.closed().fuse();

            pin_mut!(signal_closed);

            loop {
                tokio::select! {
                    result = conn.as_mut() => {
                        if let Err(_e) = result {
                            log::trace!("Failed to serve connection : {_e:#}");
                        }
                        break;
                    }
                    _ = &mut signal_closed => {
                        log::trace!("Signal received, starting graceful shutdown");
                        conn.as_mut().graceful_shutdown();
                    }
                }
            }

            drop(close_rx);
        });
    }

    drop(close_rx);
    drop(listener);

    close_tx.closed().await;

    Ok(())
}

async fn tcp_accept(listener: &TcpListener) -> Option<(TcpStream, SocketAddr)> {
    match listener.accept().await {
        Ok(conn) => Some(conn),
        Err(e) => {
            if is_connection_error(&e) {
                return None;
            }

            log::error!("Connection Accept error : {}", e);
            tokio::time::sleep(Duration::from_secs(1)).await;
            None
        }
    }
}

fn is_connection_error(error: &io::Error) -> bool {
    matches!(
        error.kind(),
        io::ErrorKind::ConnectionRefused
            | io::ErrorKind::ConnectionAborted
            | io::ErrorKind::ConnectionReset
    )
}

async fn handle_server_request(_: Request<Incoming>) -> Response<Full<Bytes>> {
    Response::new(Full::new(Bytes::from("Hello World!")))
}
