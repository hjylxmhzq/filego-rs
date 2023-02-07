cd ./server
cargo build --release
cp target/release/server ../dist/filego
cd ..
mkdir ./dist/files
