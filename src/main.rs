use pingora::server::Server;
use pingora::server::configuration::Opt;
use pingora_proxy;
use log::info;
mod gateway;

fn main() {
    std_logger::Config::logfmt().init();
    info!("hello");
        
    let mut my_server = Server::new(Opt::parse_args()).unwrap();

    my_server.bootstrap();

    let mut my_proxy = pingora_proxy::http_proxy_service(
        &my_server.configuration,
        gateway::MyGateway{}
    );

    my_proxy.add_tcp("127.0.0.1:8080");
    my_server.add_service(my_proxy);
    my_server.run_forever();
}
