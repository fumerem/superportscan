use std::net::{IpAddr, SocketAddr, TcpStream};
use std::str::FromStr;
use regex::Regex;
use std::process;
use std::time::Duration;

pub fn port_handle(port_str: &str) -> Vec<u16> {
    let port_match = Regex::new(r"(\d-\d)").unwrap();
    let mut port_scan: Vec<u16> = vec![];
    if port_match.is_match(port_str) {
        let split: Vec<&str> = port_str.split("-").collect();
        for port in split[0].parse::<u16>().unwrap()..=split[1].parse::<u16>().unwrap() {
            port_scan.push(port);
        } 
    } else if port_str == "" {
        for port in 1..=65535 {
            port_scan.push(port);
        }
    } else {
        print!("need vaild port range");
        process::exit(0);
    }
    port_scan
}

/*
#[warn(dead_code)]
pub fn full_connect(sock: SocketAddr) {
    match TcpStream::connect(sock) {
        Ok(_) => {
            println!("{:?} open", sock);
        },
        Err(_) => {}
    }
}
*/

pub fn ip_handle(ip_str: &str) -> Vec<IpAddr> {
    let mut ip_num: i32 = 0;
    let mut ip_scan: Vec<IpAddr> = vec![];
    let ip_match = Regex::new(r"(\d.\d.\d.\d)").unwrap();
    let split: Vec<&str> = ip_str.split(" ").collect();
    for _ in &split {
        ip_num += 1;
    }
    if ip_num != 0 {
        for ip in split {
            if ip_match.is_match(ip) {
                ip_scan.push(IpAddr::from_str(ip).expect("need vaild ip"));
            }
        }
    } else {
        print!("need at least one ip");
        process::exit(0);
    }
    ip_scan
}


pub fn tcp_connect(target: SocketAddr, timeout: &u64) {
    let timeout = Duration::from_secs(*timeout);
    match TcpStream::connect_timeout(&target, timeout) {
        Ok(_) => println!("{:?} open", &target),
        _ => {}
    }
}

