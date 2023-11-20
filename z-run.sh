#!/bin/bash
(sudo docker run -d -p 27017:27017 -v mongodbvol:/data/db mongo-db12)
sleep 10

cargo run --manifest-path /home/af/apix2/mongoRust/Cargo.toml --bin mongoRust
cargo run --manifest-path /home/af/apix2/webRust/Cargo.toml --bin webRust
