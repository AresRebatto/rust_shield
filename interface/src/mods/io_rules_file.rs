use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
//use serde_json::Result;
use super::rule::Rule;

pub fn load_rules_from_file() -> Vec<Rule> {
    let file_path = "../rules.json";
    let file = OpenOptions::new().read(true).open(file_path);

    match file {
        Ok(mut file) => {
            let mut contents = String::new();
            if let Err(err) = file.read_to_string(&mut contents) {
                eprintln!("Error reading file: {}", err);
                return Vec::new();
            }

            match serde_json::from_str(&contents) {
                Ok(rules) => rules,
                Err(err) => {
                    eprintln!("Error deserializing JSON: {}", err);
            
                    Vec::new()
                }
            }
        }
        Err(_) => {
            // File does not exist, create it
            if let Err(err) = File::create(file_path) {
                eprintln!("Error creating file: {}", err);
            }
            vec![Rule::new(
                super::rule::RuleKind::StandardRule,
                None,
                None,
                None,
                None,
                super::rule::Permission::Allow,
                0
            )]
            
        }
    }
}

pub fn save_rules_to_file(rules: &Vec<Rule>) -> Result<i32, &'static str> {
    let file_path = "../rules.json";
    
    // Apri il file in modalità scrittura
    let file = OpenOptions::new().write(true).truncate(true).create(true).open(file_path)
        .map_err(|_| "Error opening file")?;

    // Serializza il JSON
    let json = serde_json::to_string_pretty(rules)
        .map_err(|_| "Error serializating JSON")?;
    
    // Scrivi nel file
    let mut file = file;
    file.write_all(json.as_bytes()).map_err(|_| "Error writing file")?;

    Ok(0) // Successo
}

pub fn remove_rule_from_file(rule_id: u32) -> Result<i32, &'static str> {
    let mut rules = load_rules_from_file();
    if rule_id == 0 {
        return Err("You can't remove the standard rule");
    }

    if let Some(index) = rules.iter().position(|rule| rule.id == rule_id) {
        rules.remove(index);
        for i in index..rules.len() {
            rules[i].id = i as u32;
        }
    } else {
        return Err("Rule not found");
    }

    save_rules_to_file(&rules)?;
    Ok(0)
}
