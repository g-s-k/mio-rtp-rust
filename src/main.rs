use std::env;
use std::process;

fn main() {
    // get command line arguments
    let args: Vec<String> = env::args().collect();
    // check that all are present
    if args.len() != 5 {
        println!("usage: mio <alsa client name> <mio host address> <mio port> <local port>");
        process::exit(1);
    }
    // give the arguments clearly named identifiers
    let alsa_client_name = &args[1];
    let mio_host_address = &args[2];
    let mio_port = &args[3].parse::<u16>().unwrap();
    let own_port = &args[4].parse::<u16>().unwrap();
    // temp: print them (TODO: actually do something with them)
    println!("{} {} {} {}", alsa_client_name, mio_host_address, mio_port, own_port);
}
