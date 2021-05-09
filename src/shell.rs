use std::path::PathBuf;
use crate::{debug, input};
use crate::{
	glass,
};

const DEFAULT_SHELL: &str = "glass";
const INITIAL_CURSOR: &str = "glass: /> ";
const INITIAL_PATH: &str = "./";

pub enum OsResult {
	Success,
	Info(String),
	Error(String),
	Exit,
}

impl OsResult {
    pub fn debug(&self) {
		match self {
		    OsResult::Info(i) => println!("info: {}", i),
		    OsResult::Error(e) => println!("error: {}", e),
			_ => {},
		}
	}
}

#[derive(Debug)]
struct OsShell<'a, 'b, 'c, 'd> {
	shell: &'a mut String,
	path: &'b mut PathBuf,
	cursor: &'c mut String,
	argv: &'d mut Vec<String>,
}

impl<'a, 'b, 'c, 'd> OsShell<'a, 'b, 'c, 'd> {
	fn new(
		shell: &'a mut String,
		path: &'b mut PathBuf,
		cursor: &'c mut String,
		argv: &'d mut Vec<String>
	) -> OsShell<'a, 'b, 'c, 'd> {
		OsShell {
			shell,
			path,
			cursor,
			argv,
		}
	}
}

pub fn run() -> OsResult {
	let mut shell = String::new();
	let mut path = PathBuf::from(INITIAL_PATH);
	let mut cursor = String::from(INITIAL_CURSOR);

	loop {
		let mut argv = match shell_words::split(input(&cursor).as_str()) {
			Ok(a) => a,
			Err(error) => {
				println!("error: {}", error);
				continue;
			},
		};

		if argv.is_empty() { continue; }
		// If the shell variable has a value, prepend that to the arguments
		if !shell.is_empty() { argv.insert(0, shell.to_string()); }

		let os_shell = OsShell::new(&mut shell, &mut path, &mut cursor, &mut argv);
		
		match parse_argv(os_shell) {
		    OsResult::Exit => break,
			o => o.debug(),
		}
	}

	OsResult::Success
}

fn parse_argv(os_shell: OsShell) -> OsResult {
	debug(format!("{:?}", os_shell));

	let shell = os_shell.shell;
	let path = os_shell.path;
	let cursor = os_shell.cursor;
	let argv = os_shell.argv;

	// Glass default shell
	if shell.is_empty() {
		match argv.len() {
			1 if is_shell(&argv[0]) => {
				// sand, ash, lime
				println!("debug: &argv[0] == {}", &argv[0]);
				println!("debug: is_shell(&argv[0]) == {}", is_shell(&argv[0]));
				*shell = argv[0].clone();
				*cursor = format!("{}: {}> ", argv[0], path_display(path));
			},
			l if l >= 1 => {
				if argv[0] == "help" { glass::help().debug(); }
				else if argv[0] == "exit" || argv[0] == "quit" { return OsResult::Exit; }
			},
			_ => {},
		}
	}

	// Generic commands
	match argv.len() {
		l if l >= 2 => {
			if argv[1] == "back" || argv[1] == DEFAULT_SHELL {
				shell.clear();
				*cursor = format!("{}: {}> ", DEFAULT_SHELL, path_display(path));
			}
		},
		_ => {},
	}

	OsResult::Success
}

fn is_shell(shell: &String) -> bool { shell == "sand" || shell == "ash" || shell == "lime" }

fn path_display(path: &mut PathBuf) -> String {
	format!("{}/", path.strip_prefix(".").expect("error: Could not strip prefix of path").display())
}