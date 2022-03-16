use trillium_async_std::TcpConnector;
use trillium_logger::Logger;
use trillium_rustls::RustlsConnector;
use trillium::{Conn, Handler};

type Proxy = trillium_proxy::Proxy<RustlsConnector<TcpConnector>>;

pub fn main() {
    env_logger::init();
    trillium_async_std::run((Logger::new(), |conn: Conn| async move {
        if conn.path().starts_with("/_proxy_") {
    Proxy::new("https://httpbin.org/").run(conn).await
        } else { conn }
    }));
}
