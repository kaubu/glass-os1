use crate::shell::OsResult;

pub fn help() -> OsResult {
	println!("[GlassOS Help]
help\t\tThis command. Lists all commands available to the glass shell.
sand\t\tSystem Authority Native Daemon. This is used to login and create new users.
ash\t\tA SHell. This is the main interface. Requires authentication from SAND.
lime\t\tLocal Interface Module Execution. This is used for external programs and applications,
\t\twhich you may want to run on your GlassOS machine. Requires authentication from SAND.
exit / quit\tShuts down GlassOS.");
	OsResult::Success
}

