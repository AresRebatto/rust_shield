//use pcap::Device;
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
		"ip-address"=> println!("Comando valido"),
		"port" => println!("Comando valido"),
		"sh" => println!("Comando valido"),
		"rm-rule" => println!("Comando valido"),
		_ => return Err("Unrecognized command")
	}

	return Ok(0);
}
