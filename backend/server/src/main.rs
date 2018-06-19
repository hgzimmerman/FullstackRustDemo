extern crate server;

fn main() {

    server::setup_logging();

    let config = server::parse_arguments();

    server::init_rocket(config).launch();
}

