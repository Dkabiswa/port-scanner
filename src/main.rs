use std::env;
use std::net::IpAddr;
use std::str::FromStr;

#[derive(Debug)]
struct Arguments {
    flag: String,
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

                Ok(Self {
                    flag: String::from(""),
                    ipaddr,
                    threads: 4,
                })
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

        Ok(Self {
            flag: args[0].clone(),
            ipaddr,
            threads,
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match Arguments::new(&args) {
        Ok(config) => {
            println!("{:?}", config);
        }
        Err("help") => {
            println!("Usage:");
            println!("  scanner <IP>");
            println!("  scanner -j <THREADS> <IP>");
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
