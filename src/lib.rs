mod login_items;

use clap::{App, Arg, SubCommand};
use std::{error::Error, ptr};

const GLOBAL_FLAG_STR: &str = "-g --global 'List global login items'";

struct InternalLogger {
    log_callback: extern "C" fn(*mut u8, u64),
}

impl InternalLogger {
    fn log(&self, msg: &str) {
        (self.log_callback)(msg.as_ptr() as *mut u8, msg.len() as u64);
    }
}

#[no_mangle]
pub extern "C" fn start(
    size: u64,
    data: *mut u8,
    result_callback: extern "C" fn(*mut u8, u64),
    log_callback: extern "C" fn(*mut u8, u64),
) -> u32 {
    // Extract arguments to a vector
    let mut dst_vec = Vec::<u8>::with_capacity(size as usize);
    unsafe {
        ptr::copy(data, dst_vec.as_mut_ptr(), size as usize);
        dst_vec.set_len(size as usize);
    }
    let args_str = String::from_utf8_lossy(&dst_vec);
    // Setup logger
    let logger = InternalLogger { log_callback };

    let out: String = match parse_and_run(&args_str, &logger) {
        Ok(output) => output,
        Err(e) => format!("{}", e),
    };
    result_callback(out.as_ptr() as *mut u8, out.len() as u64);
    return 0;
}

// loginitems list
// loginitems list -g
// loginitems add name path
// loginitems add -g name path
// loginitems rm name
// login items rm -g name
fn parse_and_run(args_line: &str, logger: &InternalLogger) -> Result<String, Box<dyn Error>> {
    let mut full_line: String = "loginitems ".to_owned();
    full_line.push_str(args_line);
    let words = shellwords::split(full_line.as_str())?;
    let global: bool;
    let mut res = String::from("no result");
    let app = App::new("loginitems")
        .about("Interact with MacOS login items")
        .subcommand(SubCommand::with_name("list").arg(Arg::from_usage(GLOBAL_FLAG_STR)))
        .subcommand(
            SubCommand::with_name("add")
                .arg(Arg::from_usage(GLOBAL_FLAG_STR))
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
                .arg(Arg::from_usage(GLOBAL_FLAG_STR))
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

    let matches = app.get_matches_from_safe(words)?;
    match matches.subcommand() {
        ("list", Some(ls_cmd)) => {
            global = ls_cmd.is_present("global");
            res = login_items::list_login_items(global);
        }
        ("add", Some(add_cmd)) => {
            global = add_cmd.is_present("global");
            let path = add_cmd.value_of("path").unwrap();
            let name = add_cmd.value_of("name").unwrap();
            if !login_items::add_login_item(global, name, path) {
                logger.log(format!("[!] Failed to add login item {}", name).as_str());
            }
        }
        ("rm", Some(rm_cmd)) => {
            global = rm_cmd.is_present("global");
            let path = rm_cmd.value_of("path").unwrap();
            let name = rm_cmd.value_of("name").unwrap();
            if !login_items::rm_login_item(global, name, path) {
                logger.log(format!("[!] Failed to remove login item {}", name).as_str());
            };
        }
        _ => {
            logger.log("unknown command");
        }
    }
    Ok(res)
}
