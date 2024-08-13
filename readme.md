the repository consists of 3 binary executibles : dest_serv, server, cleint files.

these binaries in order to be run are declared in the Cargo.toml file where needed depencies are added too.
Client-> sends random log messages to server
server-> buffers the received logs from Client and flushes them based on time and size. it also forwards the log messages to the destination server and sends formatted log wrapped in needed format to elastisearch for indexing.
dest_server-> destination server receivwr forwarded data from server and prints them on console.

Commands to run
1) Cargo build-> to build all dependencies
then open three terminals and in each terminal run the following command:
2) cargo run --bin <name_of_executable>
to simulatenously run the client, server and destination server and to see them interact ssynchronously