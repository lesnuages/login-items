mod login_items;

use clap::{App, Arg, SubCommand};

const GLOBAL_FLAG_STR: &str = "-g --global 'List global login items'";

fn main() {
	let global: bool;
	// We don't put --global as a flag to the root command because
	// Grumble will consider it as a flag for its command, and won't
	// pass it as an argument to our code
	let app = App::new("loginitems")
		.about("Interact with MacOS login items")
		.arg(Arg::from_usage(GLOBAL_FLAG_STR))
		.subcommand(SubCommand::with_name("list"))
		.subcommand(
			SubCommand::with_name("add")
				.arg(
					Arg::with_name("name")
						.short("n")
						.takes_value(true)
						.value_name("NAME")
						.help("Name of the login item to add"),
				)
				.arg(
					Arg::with_name("path")
						.short("p")
						.takes_value(true)
						.value_name("PATH")
						.help("Path to the login item"),
				),
		)
		.subcommand(
			SubCommand::with_name("rm")
				.arg(
					Arg::with_name("name")
						.short("n")
						.takes_value(true)
						.value_name("NAME")
						.help("Name of the login item to add"),
				)
				.arg(
					Arg::with_name("path")
						.short("p")
						.takes_value(true)
						.value_name("PATH")
						.help("Path to the login item"),
				),
		);
	let matches = match app.get_matches_from_safe(std::env::args()) {
		Ok(m) => m,
		Err(e) => {
			println!("{}", e);
			return;
		}
	};
	global = matches.is_present("global");
	match matches.subcommand() {
		("list", Some(_)) => {
			let res = login_items::list_login_items(global);
			println!("{}", res);
		}
		("add", Some(add_cmd)) => {
			let path = add_cmd.value_of("path").unwrap();
			let name = add_cmd.value_of("name").unwrap();
			println!("Adding {}:{}", name, path);
			if !login_items::add_login_item(global, name, path) {
				println!("Failed to add login item")
			}
		}
		("rm", Some(rm_cmd)) => {
			let path = rm_cmd.value_of("path").unwrap();
			let name = rm_cmd.value_of("name").unwrap();
			println!("Removing {}:{}", name, path);
			if !login_items::rm_login_item(global, name, path) {
				println!("Failed to remove login item {}", name)
			};
		}
		_ => {
			println!("unknown command");
		}
	}
}
