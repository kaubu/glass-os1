use crate::shell::OsResult;

pub fn help() -> OsResult {
	println!("[GlassOS Help]
help\t\tThis command. Lists all commands available to the glass shell.
exit / quit\tShuts down GlassOS.");
	OsResult::Success
}

