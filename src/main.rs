use std::env;

struct CIDRBlock {
    first_address:  u32,
    last_address:   u32,
    num_addresses:  u32,
}



impl CIDRBlock {
    fn new(cidr_string: &str) -> CIDR {
        
        CIDRBlock {
            first_address: first,
            last_address: last,
            num_addresses: last-first,
        }
    }
}

fn parse_cidr_string(cidr_string: 

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        println!("The first arugment is {}", args[1]);
    }
}
