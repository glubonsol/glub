use std::io::{Write, Read};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::fs::File;
use std::path::PathBuf;


fn handle_client(mut stream: TcpStream){

    let mut buffer = [0; 1024];

    stream.read(&mut buffer).expect("Failed to read incoming request");

    let request = String::from_utf8_lossy(&buffer);
    println!("Received request: {}", request);

    let path = PathBuf::from("/home/jslinux/Code/Solana/glub_site/index.html");

    let mut file = File::open(path).expect("Could'nt open HTML file");

    let mut content = String::new();

    file.read_to_string(&mut content).expect("Failed to read HTML file");

    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n{}",
        content);

    stream.write(response.as_bytes()).expect("Failed to send response");

    stream.flush().expect("Failed to flush stream");

}



//Entry point
fn main(){

    let listener = TcpListener::bind("127.0.0.1:8088")
    .expect("Failed to bind to adress 127.0.0.1:8088");

    println!("Server listening on port 8088");

    for stream in listener.incoming(){

        match stream{

            Ok(stream)=>{
                thread::spawn(|| handle_client(stream));
            }

            Err(e)=>{
                eprintln!("Failed to establish connection, {}", e);
            }
        }
    }
}