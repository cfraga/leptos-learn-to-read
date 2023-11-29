SERVER_ADDR=192.168.12.149
SERVER_USER=cfraga
SERVER_FOLDER=web/learn-to-read/

cargo leptos build --release
rm -rf deploy/
mkdir deploy
cp target/release/learn-to-read deploy/
cp -r target/site deploy/
cp -r wordlist deploy/

scp deploy/ $SERVER_USER@$SERVER_ADDR:$SERVER_FOLDER