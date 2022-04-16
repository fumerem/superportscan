mod ports;
mod lib;
use jobpool::JobPool;
use clap::{Arg, App};
use std::rc::Rc;
use std::net::SocketAddr;


fn main() {
    let matches = App::new("superportscan")
        .version("0.2.0")
        .author("rhiood camilae<1187519105@qq.com>")
        .about("A simple tool to scan port")
        .arg(
            Arg::with_name("ip")
                .required(true)
                .index(1)
                .help("target ip| example: 1.1.1.1")
                .default_value("127.0.0.1"),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .help("target port| example: 1-1000")
                .default_value("default_port"),
        )
        .arg(
            Arg::with_name("time")
                .short("t")
                .long("time")
                .takes_value(true)
                .help("try connect time")
                .default_value("2")
        )
        .get_matches();
    
    let ips = matches.value_of("ip").unwrap();
    let ports = matches.value_of("port").unwrap();
    let timeout = matches.value_of("time").unwrap().parse::<u64>().unwrap();
    //let scan_mode = matches.value_of("mode").unwrap();
    let mut scan: Vec<SocketAddr> =vec![];
    if ports == "default_port" {
        let port_to_scan: Vec<u16> = ports::default_ports();
        for ip in lib::ip_handle(ips) {
            let i = Rc::new(ip);
            for port in &port_to_scan {
                let i = Rc::clone(&i);
                scan.push(SocketAddr::new(*i, *port))
            }
        }
    } else {
        for ip in lib::ip_handle(ips) {
            let i = Rc::new(ip);
            for port in lib::port_handle(ports) {
                let i = Rc::clone(&i);
                scan.push(SocketAddr::new(*i, port));
            }
        }
    }

    if scan.is_empty() {
        eprintln!("need vaild sock list")
    }
    
    let pool_size = 1500;
    let mut pool = JobPool::new(pool_size);

    for sock in scan {
        pool.queue(move || {
            lib::tcp_connect(sock, &timeout);
        });
    }
    pool.shutdown();

}
