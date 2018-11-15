use std::env;
use std::fs::{create_dir_all, metadata, File};
use std::process;

mod arguments;

use arguments::Arguments;

fn main() {
	let args: Vec<String> = env::args().collect();

	let arguments = Arguments::new(&args).unwrap_or_else(|_| {
		process::exit(0);
	});

	let duck_append: String = "./appState/".to_owned();
	let redux_types = vec!["Actions", "Reducers", "Sagas", "Types"];

	for i in 1..arguments.duck.len() {
		let duck = &arguments.duck[i];
		let path = duck_append.clone() + &duck;

		for file in &redux_types {
			let duck = duck.clone();
			let extension = arguments.extension.clone();
			let duck_file_path = path.clone() + &"/" + &file.clone().to_lowercase();

			create_dir_all(duck_file_path.clone()).expect("Could not create Dirs");

			let duck_file = duck_file_path + &"/" + &duck + file + &extension;

			if metadata(duck_file.clone()).is_err() {
				File::create(duck_file.clone()).expect("Something went wrong!");

				println!("ceated: {}", duck_file);
			} else {
				println!("NOT ceated: {}", duck_file);
			}
		}
	}
}
