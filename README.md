# RedLizard
RedLizard Rust TCP Reverse Shell Server/Client

This is a reverse shell in Rust called RedLizard, basically it is just a cmd.exe executing commands.
This uses SSL encryption and some basic reversing on strings.
Binary can get a bit big around 3~3.5mb since it needs to statically compile the OpenSSL library.  
  
Try it against strong EDRs, you will surprised :D  

For coming this project you need to compile from the directory of each project with the below command:    
 
`cargo build --release`  

The server can be cross compiled for Linux or Windows but the client would be better to be compiled in Windows host.  
To statically create the binary, you need to execute those commands in CMD before the previously mentioned command to compile:  

`"C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\VC\Auxiliary\Build\vcvars64.bat"`  
`set RUSTFLAGS=-C target-feature=+crt-static`  

The python script will take the client after being compiled and will replace with the IP and PORT of your choice inside the binary.  
If this will fail, you can always use the code to manually alter the IP and port as shown in my [OffensiveRust](https://github.com/trickster0/OffensiveRust) repo.  

For the client, just run it on the victim, for the Server you will need to create a crt and a key file for the SSL and have it in the current directory.  
`openssl genrsa -out ca.key 2048`  
`openssl req -new -x509 -days 3500 -key ca.key -out ca.crt`  
`openssl genrsa -out server.key 2048`  
`openssl req -new -key server.key -out server.csr`  
`openssl x509 -req -days 3500 -in server.csr -CA ca.crt -CAkey ca.key -set_serial 01 -out server.crt`  
`openssl pkcs12 -export -out server.p12 -inkey server.key -in server.crt -chain -CAfile ca.crt`  
`rm server.p12 server.csr ca.key`  

For the server just supply as parameter the port you want to run it at.

Dependencies for this project requires you to install OpenSSL and ActivePerl.
