mod mods;

use mods::standard_rule::standard_rule;
use mods::{ip_rules, port_rules, io_rules_file, standard_rule};
use std::io;
use std::io::Write;
fn main() {
    println!("Configure firewall");
    loop{
    	
		print!(">> ");
		
		let mut input = String::new();
    	io::stdout().flush().unwrap();
    	io::stdin()
    		.read_line(&mut input)
    		.expect("Error");

    	match execute_command(&input){
    		Ok(_)=> println!("The rule was sucessfully inserted"),
    		Err(e) => println!("Error: {}", e)
    	}
    }
}

fn execute_command(_command: &str)-> Result<i32, &str>{

	
	let command_param: Vec<&str> = _command.split_whitespace().collect();
	
	match command_param[0]{
		"ip-address"=> ip_rules::ip_rule(command_param),
		"port" => Ok(0),
		"sh" => Ok(0),
		"rm-rule" => Ok(0),
		"as-standard" => standard_rule::standard_rule(command_param),
		_ => return Err("Unrecognized command")
	}
}



