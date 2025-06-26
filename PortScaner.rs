use std::io::{self, Write};

fn main()
{
    print!("scan IP: ");
    io::stdout().flush().unwrap();
    let mut ip = String::new();

    io::stdin()
        .read_line(&mut ip)
        .expect("invalid read");

    let ip = ip.trim();
    println!("IP digitado: {}", ip);
}