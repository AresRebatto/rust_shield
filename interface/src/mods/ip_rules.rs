use std::net::Ipv4Addr;
use std::str::FromStr;
use serde::{Serialize, Deserialize}; 
use serde_json;

///Translates and verifies the rule related to IP address. 
/// If it's valid, then it saves it, otherwise it returns 
/// error;
/// ```
/// return Ok(0);
/// return  Err("If you use the --range parameter, you have to enter a range in the format start-address:end-address");
/// return Err("Command isn't valid");
/// return Err("Ip range must have only two Ip Addresses");
/// return Err("Subnetmask isn't valid");
/// return Err("Subnet mask isn't valid");
/// return Err("Ip address isn't valid");
/// ```
pub fn ip_rule(_command_params: Vec<&str>)-> Result<i32, &str>{

    //It is used to identify what the subnetmask parameter is
    let mut sb_index:usize = 0;
    
	if _command_params.len() < 4{
        return Err("Command isn't valid");
    }

    if _command_params[1] == "--range"{

        sb_index = 3;

        if !_command_params[2].contains(':')
        {
            return  Err("If you use the --range parameter, you have to enter a range in the format start-address:end-address");
        }

        let range: Vec<&str>= _command_params[2].split(":").collect();

        if range.len()>2{
            return Err("Ip range must have only two Ip Addresses");
        }

        match (verify_ip(range[0]), verify_ip(range[1])) {
            (Ok(ip1), Ok(ip2)) => {

                
            },
            _ => return Err("One of the ip addresses isn't valid"),
        }
    }else{
        sb_index = 2;

        match verify_ip(_command_params[1]) {
            Ok(ip) => {

                
            },
            Err(e)=> return Err(e),
        }
    }

    match subnet_mask_to_prefix(_command_params[sb_index]){
        Ok(prefix)=>{

        },
        Err(e) => return Err(e),
    }

    return Ok(0);
}

pub fn verify_ip(ip_address: &str)-> Result<&str, &str>{
    if Ipv4Addr::from_str(ip_address).is_ok(){
        return Ok(ip_address);
    }

    return Err("Ip address isn't valid")
}

fn subnet_mask_to_prefix(mask: &str) -> Result<u8, &str> {
    if let Ok(ip) = Ipv4Addr::from_str(mask) {
        return Ok(ip.octets().iter().map(|&octet| octet.count_ones()).sum::<u32>() as u8);
    }
    
    Err("Subnet mask isn't valid")
    
}