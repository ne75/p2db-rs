extern crate serialport;

use clap::Parser;

mod tools;
mod db_server;
use db_server::DBServer;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The app executable to debug
    app: String,
    /// The serial port of the connected P2 to run on
    port: String
}

fn main() {
    let args = Args::parse();
    let mut db_serv = DBServer::new(args.port, args.app);
    
    db_serv.start();

    loop {
    }
}
