extern crate argparse;
extern crate mogilefsd;

use argparse::ArgumentParser;
use std::collections::HashMap;
use std::default::Default;
use std::net::{TcpListener, Ipv4Addr};
use std::thread;
use mogilefsd::tracker;
use std::sync::Arc;

type CommandArgs = HashMap<String, Vec<String>>;
type TrackerResult = Result<String, String>;

fn main() {
    let mut opts: Options = Default::default();
    opts.parser().parse_args_or_exit();
    let listener = TcpListener::bind((opts.listen_ip, opts.listen_port)).unwrap();
    let handler = Arc::new(tracker::Handler::new());

    for stream_result in listener.incoming() {
        let local_handler = handler.clone();
        match stream_result {
            Ok(stream) => {
                thread::spawn(move|| local_handler.handle(stream));
            },
            Err(e) => {
                panic!("Connection failed: {:?}", e);
            },
        }
    }
}

#[derive(Debug)]
struct Options {
    listen_ip: Ipv4Addr,
    listen_port: u16,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            listen_ip: Ipv4Addr::new(0, 0, 0, 0),
            listen_port: 7002,
        }
    }
}

impl Options {
    fn parser(&mut self) -> ArgumentParser {
        let mut parser = ArgumentParser::new();
        parser.set_description("A partial clone for the MogileFS tracker daemon.");

        parser.refer(&mut self.listen_ip).add_option(
            &[ "-l", "--listen-ip" ],
            argparse::Store,
            "The host IP for the tracker to listen on.");

        parser.refer(&mut self.listen_port).add_option(
            &[ "-l", "--listen-ip" ],
            argparse::Store,
            "The port for the tracker to listen on.");

        parser
    }
}

// fn handle_client(stream: TcpStream) {
//     let mut handler = tracker::Handler::new();
//     handler.handle(stream);
// }
