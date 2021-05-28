use async_net::TcpStream;
use trillium_client::Rustls;
use trillium_logger::DevLogger;
use trillium_proxy::Proxy;

pub fn main() {
    env_logger::init();
    trillium_smol_server::run((
        DevLogger,
        Proxy::<Rustls<TcpStream>>::new("https://httpbin.org/"),
    ));
}