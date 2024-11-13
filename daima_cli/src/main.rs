use std::{
    io::{self, Write},
    os::unix::net::UnixStream,
};

fn main() {
    let mut skipped_program = std::env::args().skip(1);
    let socket = skipped_program.next().unwrap();
    println!("socket: {socket}");

    let mut stream = UnixStream::connect(socket).unwrap();
    loop {
        println!("Next input line");
        let mut next_line = String::new();
        io::stdin().read_line(&mut next_line).unwrap();
        if next_line.trim() == "exit" {
            stream.shutdown(std::net::Shutdown::Both).unwrap();
            break;
        } else {
            stream.write_all(next_line.as_bytes()).unwrap();
            stream.write_all(&[b'\0']).unwrap();
        }
    }
}
