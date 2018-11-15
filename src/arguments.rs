pub struct Arguments {
	pub duck: Vec<String>,
	pub extension: String,
}

impl Arguments {
	pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
		let help_message = "Usage: duckName1 duckNam2[ -js]
			\r\n -js for JS files, default is TS
			\r\n -h --help to show this message";

		if args.len() < 1 {
			println!("{}", help_message);

			return Err("Please provide a DUCK name!");
		} else if args.clone().iter().any(|x| x == "-js") {
			return Ok(Arguments {
				duck: args
					.iter()
					.filter(|arg| !arg.contains("-js"))
					.cloned()
					.collect(),
				extension: ".js".to_string(),
			});
		} else if args.clone().iter().any(|x| x == "-h" || x == "--help") {
			println!("{}", help_message);
			return Err("help");
		} else {
			return Ok(Arguments {
				duck: args.to_vec(),
				extension: ".ts".to_string(),
			});
		}
	}
}
