use serde::{Serialize, Deserialize};

use super::port_rules;

#[derive(Serialize, Deserialize)]
pub struct Rule{
    pub id: u32,
    pub rule_kind: RuleKind,
    pub start_address: Option<String>,
    pub end_address: Option<String>,
    pub subnet_mask: Option<u8>,
    pub port: Option<u16>,
    pub permission: Permission
}
impl Rule{
    
    pub fn new(
        _rule_kind: RuleKind, 
        _start_address: Option<&str>, 
        _end_address: Option<&str>, 
        _subnet_mask: Option<u8>,
        _port: Option<u16>,
        _permission: Permission,
        _last_id: u32
    )-> Self{
        match _rule_kind{
            RuleKind::IpAddressRule => {
                if _end_address.is_none(){
                    return Self { 
                        id: _last_id+1,
                        rule_kind: _rule_kind, 
                        start_address: Some(String::from(_start_address.unwrap())), 
                        end_address: None, 
                        subnet_mask: _subnet_mask, 
                        port: _port, 
                        permission: _permission 
                    }
                }
        
                return Self { 
                    id: _last_id+1,
                    rule_kind: _rule_kind, 
                    start_address: Some(String::from(_start_address.unwrap())), 
                    end_address: Some(String::from(_end_address.unwrap())), 
                    subnet_mask: _subnet_mask, 
                    port: _port, 
                    permission: _permission 
                }
            },
            RuleKind::PortRule => {
                return Self { 
                    id: _last_id+1,
                    rule_kind: _rule_kind, 
                    start_address: None, 
                    end_address: None, 
                    subnet_mask: None, 
                    port: _port, 
                    permission: _permission 
                }
            },
            RuleKind::StandardRule => {
                return Self { 
                    id: 0,
                    rule_kind: RuleKind::StandardRule, 
                    start_address: None, 
                    end_address: None, 
                    subnet_mask: None, 
                    port: None, 
                    permission: _permission 
                }
            }
        }
        
    }
}

#[derive(Serialize, Deserialize)]
pub enum RuleKind{
    IpAddressRule,
    PortRule,
    StandardRule
}

#[derive(Serialize, Deserialize)]
pub enum Permission{
    Allow,
    Deny
}