
use std::net::*;
use std::str::FromStr;
use std::time::Duration;
use std::env;

use crossbeam::thread;

// rfc1112: "The All Hosts multicast group addresses all hosts on the same network segment." (https://tools.ietf.org/rfc/rfc1112.txt)
const STATCOM_MULTICAST_4_GRP: &'static str = "224.0.0.1";
// rfc4291: "All Nodes Address" (https://tools.ietf.org/html/rfc4291)
const STATCOM_MULTICAST_6_GRP: &'static str = "ff02::1";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        listen_for_events();
    }
    else {
        // concatinate arguments + push over network
        let mut event_name = String::new();
        for arg in &args[1..] {
          event_name.push_str(&arg);
          event_name.push_str(" ");
        }
        event_name.pop(); // removes last " "
        send_event(&event_name);
    }
}

fn send_event(event: &str) {
  println!("event={}|", event);
}

fn listen_for_events() {
    thread::scope(|s| {
        
        s.spawn(|_| {
            listen_to_ipv4();
        });
        s.spawn(|_| {
            listen_to_ipv6();
        });

    }).expect("Crossbeam scoped thread broke");
}

fn listen_to_ipv4() {
  let socket = UdpSocket::bind("0.0.0.0:0").expect("Could not bind to unspecified UDP socket");
  socket.join_multicast_v4(
      &Ipv4Addr::from_str(STATCOM_MULTICAST_4_GRP).unwrap(),
      &Ipv4Addr::UNSPECIFIED,
  ).expect("Could not join multicast group ");
  let mut buf = [0; 10];
  let mut c = 0;
  loop {
      if let Ok((number_of_bytes, src_addr)) = socket.recv_from(&mut buf) {

          let filled_buf = &mut buf[..number_of_bytes];

          println!("I am the 2nd socket");
          println!("Message received from address = {:?}", src_addr);
          println!("Contents of the message = {:?}\n---\n", filled_buf);

          std::thread::sleep(Duration::from_millis(1));

          c += 1;
          if c == 5 {
              break;
          }
      }
  }
}

fn listen_to_ipv6() {
  let socket = UdpSocket::bind(":::0").expect("Could not bind to unspecified UDP socket");
  socket.join_multicast_v6(
      &Ipv6Addr::from_str(STATCOM_MULTICAST_6_GRP).unwrap(),
      0,
  ).expect("Could not join multicast group ");
  let mut buf = [0; 10];
  let mut c = 0;
  loop {
      if let Ok((number_of_bytes, src_addr)) = socket.recv_from(&mut buf) {

          let filled_buf = &mut buf[..number_of_bytes];

          println!("I am the 2nd socket");
          println!("Message received from address = {:?}", src_addr);
          println!("Contents of the message = {:?}\n---\n", filled_buf);

          std::thread::sleep(Duration::from_millis(1));

          c += 1;
          if c == 5 {
              break;
          }
      }
  }
}
