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
	let redux_paths = vec!["actions", "reducers", "sagas", "queries", "types"];
	let redux_files = vec!["Actions", "Reducer", "RequestSaga", "Queries", "Types"];

	for i in 1..arguments.duck.len() {
		let duck = &arguments.duck[i];
		let path = duck_append.clone() + &duck;

		for path_index in 0..redux_paths.len() {
			let duck = duck.clone();
			let extension = arguments.extension.clone();
			let duck_file_path = path.clone() + &"/" + &redux_paths[path_index];

			create_dir_all(duck_file_path.clone()).expect("Could not create dirs");

			let duck_file = duck_file_path + &"/" + &duck + redux_files[path_index] + &extension;

			if metadata(duck_file.clone()).is_err() {
				File::create(duck_file.clone()).expect("Something went wrong!");

				println!("created: {}", duck_file);
			} else {
				println!("NOT created: {}", duck_file);
			}
		}
	}
}
