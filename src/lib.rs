//! A crate with bunch of lazy macros

/// Nacro to exectute subprocess.
///
/// Arguments:
///
///* ```cmd``` - to specify command name.
///
///Usage:
///
///* ```exec_cmd!(cmd=>"cmd_name", [arg1, arg2, ..., argN]) ```
#[macro_export]
macro_rules! exec_cmd {
    (cmd=>$cmd:expr, $($arg:expr),*) => { std::process::Command::new($cmd)$(.arg($arg))* }
}

/// Macro to exectute subprocess silently.
///
/// All output will redirected to null.
///
/// Arguments:
///
///* ```cmd``` - to specify command name.
///
///Usage:
///
///* ```exec_cmd!(cmd=>"cmd_name", [arg1, arg2, ..., argN]) ```
#[macro_export]
macro_rules! exec_silent_cmd {
    (cmd=>$cmd:expr, $($arg:expr),*) => { std::process::Command::new($cmd)
                                                                 .stderr(std::process::Stdio::null())
                                                                 .stdout(std::process::Stdio::null())
                                                                 $(.arg($arg))* }
}

/// Macro to check if given path/string belongs to file.
///
/// Returns false if no such file exists or cannot access it.
#[macro_export]
macro_rules! is_file {
    ($path:expr) => { std::fs::metadata($path).ok().map_or(false, |data| data.is_file()) }
}

/// Macro to check if given path/string belongs to directory.
///
/// Returns false if no such directory exists or cannot access it.
#[macro_export]
macro_rules! is_dir {
    ($path:expr) => { std::fs::metadata($path).ok().map_or(false, |data| data.is_dir()) }
}
