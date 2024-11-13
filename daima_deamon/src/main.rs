use std::{
    io::{BufReader, Read},
    net::Shutdown,
    os::unix::net::{UnixListener, UnixStream},
    thread::scope,
};

use daima_common::{AppResult, FileLockOneProcessOnly};
use daima_deamon::constants;
use log::{error, info, warn};

fn main() -> AppResult {
    env_logger::init();

    let locked = FileLockOneProcessOnly::new_for_daemon()?;
    locked.write_own_pid_to_lock_file()?;

    socket_loop().unwrap();

    println!("Hello, world!");

    Ok(())
}

fn socket_loop() -> AppResult {
    let tmp_path = std::env::temp_dir().join(format!("{}.socket", constants::APP_NAME));
    info!("Sockete path at {:?}", &tmp_path);
    let _ = std::fs::remove_file(&tmp_path);
    let listener = UnixListener::bind(tmp_path.clone())?;
    info!("Binding to listener at {:?}", listener.local_addr());
    scope(|s| loop {
        match listener.accept() {
            Ok((stream, addr)) => {
                info!("New incomming connection with addr {:?}", &addr);

                s.spawn(|| handle_stream(stream));
            }
            Err(failed) => error!("Failed to accept connection: {}", failed),
        }
    });
    Ok(())
}

fn handle_stream(stream: UnixStream) {
    let mut next_read = BufReader::new(&stream);
    let mut buffer = [0; 100];
    loop {
        match next_read.read(&mut buffer) {
            Ok(0) => {
                info!("End of file reached");
                break;
            }
            Ok(bytes_read) => {
                info!("Bytes read: {}", bytes_read);
                match String::from_utf8(buffer.to_vec()) {
                    Ok(text) => info!("{}", &text),
                    Err(_error) => {
                        warn!("Could not convert message to string");
                        info!("Shown here as bytes {:?}", &next_read);
                    }
                }
                buffer[0..bytes_read].fill(0);
            }
            Err(error) => {
                error!(
                    "Stoping thread for reading stream because of the following error {}",
                    error
                );
                break;
            }
        }
    }
    info!("Shutdowning down stream");
    stream.shutdown(Shutdown::Both).unwrap();
}
