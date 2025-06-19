use std::{
    error,
    io::{BufRead, Write},
};

#[derive(Debug)]
enum Error {
    OpenError(std::io::Error),
    ReadError(std::io::Error),
}

fn read_file() -> Result<(), Error> {
    let file = std::fs::File::open("out.txt").map_err(Error::OpenError)?;
    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
        let line = line.map_err(Error::ReadError)?;
        println!("{}", line);
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    read_file()
    // let path = std::env::var("PATH").unwrap();
    // println!("PATH is {:?}", path);
    // let folders = std::env::split_paths(&path);
    // for folder in folders {
    //     println!("{:?}", folder);
    // }
    // let args = std::env::args();
    // for arg in args {
    // //println!("Argument: {}", arg);
    // let _= std::io::stdout().write_all(format!("Argument: {}\n", arg).as_bytes());
    // }
    // let mut input = String::new();
    // let n = std::io::stdin().read_line(&mut input).unwrap();
    // println!("you said {:?} ({} characters)", input, n);
    // let file = std::fs::File::open("out.txt").unwrap();
    // let mut new_file = std::fs::File::create("new.txt").unwrap();
    // let reader = std::io::BufReader::new(file);
    // for line in reader.lines() {
    //     let line = line.unwrap();
    //     let _ = new_file.write_all(format!("new: {}\n", line).as_bytes());
    //     println!("{}", line);
    // }
    // let receiver = match std::net::UdpSocket::bind("127.0.0.1:0") {
    //     Ok(receiver) => receiver,
    //     Err(error) => {
    //         println!("There was some error: {:?}", error);
    //         return;
    //     }
    // };
    // println!("We bound port {} for listening", receiver.local_addr().unwrap().port());

    // let sender = std::net::UdpSocket::bind("127.0.0.1:0").unwrap();
    // println!("We bound port {} for sending", sender.local_addr().unwrap().port());
    // let _ = sender.send_to(b"Hello, World!", receiver.local_addr().unwrap());

    // let mut buffer = [0; 256];
    // let (n, sender_addrss) = receiver.recv_from(&mut buffer).unwrap();
    // println!("We received {:?} from {:?}", String::from_utf8_lossy(&buffer[0..n]), sender_addrss);
}
