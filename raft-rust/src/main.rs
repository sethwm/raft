use std::env;
use std::net::SocketAddr;

use crate::app::Color;
use crate::app::ColorStateMachine;
use crate::app::piclient::PiClient;
use crate::app::piclient::SenseHat;
use crate::log_impl::in_memory::InMemoryLog;
use crate::network::real::RealNetwork;
use crate::raft::membership::Endpoint;
use crate::raft::membership::Membership;
use crate::startup::server::setup_node;
use crate::test_cluster::run_for_a_while;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

pub mod test_cluster;

pub mod app;
pub mod log_impl;
pub mod network;
pub mod raft;
pub mod startup;

// Eventually we might want to use optparse or whatever rust's best equivalent is.  For now there's
// two ways to start this:
//
// raft server <endpoint> <membership> <id for pi stripe> <x coord for pi> <y coord for pi> <pi addr>
// e.g. raft server 127.0.0.1:10001 "127.0.0.1:10001, 127.0.0.1:10002, 127.0.0.1:10003" 1 0 2 10.0.1.2:12345
// e.g. raft server 127.0.0.1:10001 127.0.0.1:10001,127.0.0.1:10002,127.0.0.1:10003 1 0 2 10.0.1.2:12345 # same deal with spaces
fn main() {

    let args: Vec<_> = env::args().collect();

    // no args (other than the command itself)... just run the test cluster
    if args.len() == 1 {
        println!("{}", serde_json::to_string(&Color::new(123, 200, 6)).unwrap());
        run_for_a_while();
        return;
    }

    println!("starting rust raft server {:?}", args);

    let mode = args[1].clone();
    if mode == "server" {
        let endpoint = Endpoint(args[2].clone());
        let membership = Membership::parse(args[3].clone());

        let id: u8 = args[4].parse().unwrap();
        let coords: (u8, u8) = (args[5].parse().unwrap(), args[6].parse().unwrap());
        let pi_addr: SocketAddr = args[7].parse().expect("cannot parse pi addr");
        let sense_hat = SenseHat(pi_addr);
        let piclient = PiClient::new(id, coords, sense_hat);
        let application = ColorStateMachine::new(piclient);

        let log = InMemoryLog::new();

        let network = RealNetwork::<Color>::new(endpoint.clone());
        setup_node(endpoint, membership, &network, application, log);

        network.run();
    } else {
        panic!("invalid arguments: {:?}", args);
    }
}
