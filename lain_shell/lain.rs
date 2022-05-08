/* Lain Shell
   	by rav3ndust
-------------------------------------------------------------
A shell which aims to be simple and fast. 
			~close the world, enter the nExt~
------------------------------------------------------------*/ 
// handle imports
use std::io::Write;
use std::env;
use std::path::Path;
use std::io::stdin;
use std::io::stdout;
use std::process::Command;
use std::thread::spawn;
// entrypoint
fn main(){
	const LAIN_PREPEND: &str = "lain shell v0.1";
	const LAIN_PROMPT: &str = "~>: "; 
	println!("{}", LAIN_PREPEND); 
	loop {  
		print!("{}", LAIN_PROMPT); 
		stdout().flush();
		let mut shell_prompt = String::new();
		stdin().read_line(&mut shell_prompt).unwrap();	
		// remove trailing newline left behind by readline().
		// treat commands after whitespace as further args.
		let mut flags = shell_prompt.trim().split_whitespace();
		let shell_command = flags.next().unwrap();
		let arguments = flags;
		// build in support for further commands
		match shell_command {
			"cd" => {
				// treat / as dir if not given
				let dir = arguments.peekable().peek().map_or("/", |x| *x);
				let root = Path::new(dir); 
				if let Err(error) = env::set_current_dir(&root) {
					eprintln!("{}", error); 
				}
			},
			"exit" => return,
		// continue
		process => {
			let process = Command::new(shell_command)
			.args(arguments)
			.spawn();
			// err handling for malformed input
			match process {
				Ok(mut process) => { process.wait(); },
				Err(error) => eprintln!("{}", error),
			};
		}
		}
	}
}
