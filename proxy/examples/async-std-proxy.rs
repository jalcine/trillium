use trillium_async_std::TcpConnector;
use trillium_logger::Logger;
use trillium_rustls::RustlsConnector;

type Proxy = trillium_proxy::Proxy<RustlsConnector<TcpConnector>>;

pub fn main() {
    env_logger::init();
    trillium_async_std::run((Logger::new(), Proxy::new("https://httpbin.org/")));
}
