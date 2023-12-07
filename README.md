# RedLizard NG
### Forked from: https://github.com/trickster0/RedLizard


#### Changes from the original project:

- Use native_tls instead of openssl, since openssl on windows it's a pain

- Client and server run both on windows and linux

- Client on linux execute command through bash

- The certificate (and the private key) is now embedded into the server as PFX file

- The client accept the server IP and port as parameter

- Adjusted the code to suppress warning

 

#### Requirements:

- rustc & cargo 1.73.0

- on linux: OpenSSL 3.0.11 with libssl-dev

