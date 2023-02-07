cd ./server
cargo build --release
cp target/release/server ../dist/server
cd ..
mkdir ./dist/files
