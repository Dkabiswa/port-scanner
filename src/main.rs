use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::process;
use std::str::FromStr;
use std::sync::mpsc::{Sender, channel};
use std::thread;

const MAX: u16 = 65535;

#[derive(Debug)]
struct Arguments {
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Self, &'static str> {
        match args.len() {
            2 => Self::parse_single(&args[1]),
            4 => Self::parse_flagged(&args[1..]),
            _ => Err("invalid number of arguments"),
        }
    }

    fn parse_single(arg: &str) -> Result<Self, &'static str> {
        match arg {
            "-h" | "-help" => Err("help"),
            _ => {
                let ipaddr = IpAddr::from_str(arg).map_err(|_| "not a valid IP address")?;

                Ok(Self { ipaddr, threads: 4 })
            }
        }
    }

    fn parse_flagged(args: &[String]) -> Result<Self, &'static str> {
        if args[0] != "-j" {
            return Err("unknown flag");
        }

        let threads = args[1]
            .parse::<u16>()
            .map_err(|_| "failed to parse thread count")?;

        let ipaddr = IpAddr::from_str(&args[2]).map_err(|_| "not a valid IP address")?;

        Ok(Self { ipaddr, threads })
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if (MAX - port) <= num_threads {
            break;
        }
        port += num_threads;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let program = args[0].clone();
    let arguments = match Arguments::new(&args) {
        Ok(config) => config,
        Err("help") => {
            println!("Usage:");
            println!("  scanner <IP>");
            println!("  scanner -j <THREADS> <IP>");
            process::exit(0);
        }
        Err(err) => {
            eprintln!("{} problem parsing arguments: {}", program, err);
            process::exit(1);
        }
    };

    let num_threads = arguments.threads;
    let addr = arguments.ipaddr;
    let (tx, rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, addr, num_threads);
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }

    println!("");
    out.sort();
    for v in out {
        println!("{} is open", v);
    }
}
