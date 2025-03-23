use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Rule{
    rule_kind: RuleKind,
    start_address: Option<String>,
    end_address: Option<String>,
    subnet_mask: Option<u8>,
    port: Option<u16>,
    permission: Permission
}

#[derive(Serialize, Deserialize)]
enum RuleKind{
    IpAddressRule,
    PortRule
}

#[derive(Serialize, Deserialize)]
enum Permission{
    Allow,
    Deny
}