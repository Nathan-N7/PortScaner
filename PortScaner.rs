use std::io::{self, Write};
use std::net::TcpStream;
use std::time::Duration;

fn scan_port(ip: &str)
{
    for port in 8080..=8081
    {
        let addr = format!("{}:{}", ip, port); //join
        if let Ok(_) = TcpStream::connect_timeout(&addr.parse().unwrap(), Duration::from_millis(100))
        {
            println!("port {} is open", port);
        }
    }
    println!("end of execution");    
}

fn main()
{
    print!("scan IP: ");
    io::stdout().flush().unwrap();
    let mut ip = String::new();

    io::stdin()
        .read_line(&mut ip)
        .expect("invalid read");
    let ip = ip.trim();
    scan_port(ip);
}