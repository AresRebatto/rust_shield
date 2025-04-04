use serde::{Serialize, Deserialize}; 
use serde_json;
use super::{rule::{Rule, RuleKind, Permission}, io_rules_file::*};
use std::fs::File;
use std::io::Write;

///Change the standard rule
pub fn standard_rule(_command_params: Vec<&str>)-> Result<i32, &str>{
    match _command_params[1] {
        "allow" => {
            let mut rules = load_rules_from_file();
            rules[0].permission = Permission::Allow;
            save_rules_to_file(&rules)?;
            return Ok(0);
        },
        "deny" => {
            let mut rules = load_rules_from_file();
            rules[0].permission = Permission::Deny;
            save_rules_to_file(&rules)?;
            return Ok(0);
        },
        _ => return Err("Command isn't valid"),
        
    }
}