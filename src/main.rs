use std::env;

struct CIDRBlock {
    first_octet: u8,
    second_octet: u8,
    third_octet: u8,
    fourth_octet: u8,
    net_mask: u8 
}

// difference between each octet and 256
fn max_val(num_bits: u32) -> u32 {
    let exp = num_bits-1;
    if exp == 0 {
        return 1;
    } else {
        return 2u32.pow(exp) + max_val(num_bits-1);
    }
}

fn to_u8(val: &str) -> u8 {
    return val.parse::<u8>().unwrap();   
}

impl CIDRBlock {
    fn new(cidr_string: &str) -> CIDRBlock {
        let res: Vec<&str> = cidr_string.split("/").collect();
        let octets: Vec<&str> = res[0].split(".").collect();
        CIDRBlock {
            first_octet: to_u8(octets[0]),
            second_octet: to_u8(octets[1]),
            third_octet: to_u8(octets[2]),
            fourth_octet: to_u8(octets[3]),
            net_mask: to_u8(res[1]),
        }

    }

    fn show(&self) {
        println!("{}.{}.{}.{}/{}", self.first_octet, self.second_octet, self.third_octet, self.fourth_octet, self.net_mask);
    }

    fn num_addresses(&self) -> u32 {
        let net_mask = self.net_mask as u32;        
        if self.net_mask < 8 {
            return max_val(net_mask)+1 * 256 * 256 * 256
        }  
        else if self.net_mask < 16 {
            return max_val(16-net_mask)+1 * 256 * 256
        }
        else if self.net_mask < 24 {
            return max_val(24-net_mask)+1 * 256
        }
        else {
            return max_val(32-net_mask)+1
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        let cidr_string = &args[1];
        let cidr = CIDRBlock::new(cidr_string);
        cidr.show();
        println!("Num Addresses: {}", cidr.num_addresses())
    }
}
