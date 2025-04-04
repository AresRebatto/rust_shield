use std::net::Ipv4Addr;
use std::str::FromStr;
use serde::{Serialize, Deserialize}; 
use serde_json;
use super::{rule::{Rule, RuleKind, Permission}, io_rules_file::*};
use std::fs::File;
use std::io::Write;


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
/// return Err("Port isn't valid")
/// ```
pub fn ip_rule(_command_params: Vec<&str>)-> Result<i32, &str>{

    //It is used to identify what the subnetmask parameter is
    let mut sb_index:usize = 0;
    let mut rule: Rule = Rule::new(
        RuleKind::IpAddressRule,
        None, //start_address
        None, //end_address
        None, //subnet_mask
        None, //port
        Permission::Allow,
        0, //last_id
    );
    
    if _command_params.len() < 4 {
        return Err("Command isn't valid");
    }

    if _command_params[1] == "--range" {
        sb_index = 3;

        if !_command_params[2].contains(':') {
            return Err("If you use the --range parameter, you have to enter a range in the format start-address:end-address");
        }

        let range: Vec<&str> = _command_params[2].split(":").collect();

        // Check if the range contains more than two IP addresses
        if range.len() > 2 {
            return Err("Ip range must have only two Ip Addresses");
        }

        // Verify the start and end IP addresses
        match (verify_ip(range[0]), verify_ip(range[1])) {
            (Ok(ip1), Ok(ip2)) => {
                rule.start_address = Some(String::from(ip1));
                rule.end_address = Some(String::from(ip2));
            },
            _ => return Err("One of the ip addresses isn't valid"),
        }
    } else {
        sb_index = 2;

        // Verify the single IP address
        match verify_ip(_command_params[1]) {
            Ok(ip) => {
                rule.start_address = Some(String::from(ip));
                rule.end_address = Some(String::from(ip));
            },
            Err(e) => return Err(e),
        }
    }

    // Convert the subnet mask to prefix length
    match subnet_mask_to_prefix(_command_params[sb_index]) {
        Ok(prefix) => {
            rule.subnet_mask = Some(prefix);
        },
        Err(e) => return Err(e),
    }

    // Check if there are additional parameters for port and permission
    if _command_params.len() == sb_index + 3 {
        match _command_params[sb_index + 1] {
            "--p" => {
                // Parse the port number
                match _command_params[sb_index + 1].parse::<u16>() {
                    Ok(port) => {
                        rule.port = Some(port);
                    },
                    Err(_) => return Err("Port isn't valid"),
                }
            },
            _ => {
                //qui da sistemare la porta
            },
        }
    } else {
        
        // Check if there is a permission parameter
        if _command_params.len() > sb_index + 1 {
            match _command_params[sb_index + 1] {
                "allow" => {
                    rule.permission = Permission::Allow;
                },
                "deny" => {
                    rule.permission = Permission::Deny;
                },
                _ => {return Err("Command isn't valid")},
            }
        } else {
            return Err("Command isn't valid");
        }
    }
    
    let mut rules = load_rules_from_file();
    rule.id = rules.len() as u32 + 1;

    rules.push(rule);
    match save_rules_to_file(&rules){
        Ok(_)=> return Ok(0),
        Err(e) => return Err(e)
    }
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