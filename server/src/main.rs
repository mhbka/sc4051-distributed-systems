use std::net::UdpSocket;
use clap::Parser;
use handler::Handler;
use socket::SenderReceiver;
use tracing::Level;

mod facilities;
mod handler;
mod log;
mod socket;

/// The server for the project.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The address to bind to
    #[arg(short, long, default_value_t = String::from("127.0.0.1:34524"))]
    addr: String,
    /// Whether to enable response caching
    #[arg(short, long, default_value_t = true)]
    use_reliability: bool
}

fn main() {
    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();

    tracing::info!("Server arguments: {args:?}");

    let socket = UdpSocket::bind(&args.addr).unwrap();
    let sender_receiver = SenderReceiver::new(socket, args.use_reliability);
    let mut handler = Handler::new(sender_receiver);

    handler.run();
}
