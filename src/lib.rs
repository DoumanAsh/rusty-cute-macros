//! A crate with bunch of lazy macros
#[cfg(feature = "time")]
extern crate time;

///Macro to construct subprocess command
///
///# Arguments:
///
///* ```cmd``` - to specify command name.
///
///# Usage:
///
///* ```exec_cmd!(cmd=>"cmd_name", [arg1, arg2, ..., argN]) ```
#[macro_export]
macro_rules! cmd {
    (cmd=>$cmd:expr, $($arg:expr),*) => { std::process::Command::new($cmd)$(.arg($arg))* }
}

///Macro to exectute subprocess silently.
///
///All output will be redirected to null.
///
///# Arguments:
///
///* ```cmd``` - to specify command name.
///
///# Usage:
///
///* ```exec_cmd!(cmd=>"cmd_name", [arg1, arg2, ..., argN]) ```
///
///Returns status code
#[macro_export]
macro_rules! exec_silent_cmd {
    (cmd=>$cmd:expr, $($arg:expr),*) => { std::process::Command::new($cmd)
                                                                 .stderr(std::process::Stdio::null())
                                                                 .stdout(std::process::Stdio::null())
                                                                 $(.arg($arg))*
                                                                 .status().unwrap() }
}

/// Macro to check if given path/string belongs to file.
///
/// Returns false if no such file exists or cannot access it.
#[macro_export]
macro_rules! is_file {
    ($path:expr) => { std::fs::metadata($path).ok().map_or(false, |data| data.is_file()); };
}

/// Macro to check if given path/string belongs to directory.
///
/// Returns false if no such directory exists or cannot access it.
#[macro_export]
macro_rules! is_dir {
    ($path:expr) => { std::fs::metadata($path).ok().map_or(false, |data| data.is_dir()); };
}

///Trace macro like ```println```
///
///It uses ```format_args!``` for creating formatted string from passed arguments.
///
///Prints with the following format: ```file!:line! - [Message]```
///
///# Time feature
///By adding ```time``` crate and enabling feature ```time```
///the date in following format will be used: ```[dd/mt/yy hh-mm-ss.ms]```
///
///Example of enabling:
///```rusty-cute-macros = { version = "*", features = ["time"] }```
#[cfg(not(feature = "time"))]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)+) => {
        println!("{}", format_args!("{}:{} - {}", file!(), line!(), format_args!($($arg)+)));
    }
}

#[cfg(feature = "time")]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)+) => {
        println!("{}", format_args!("[{}] {}:{} - {}", time::now().strftime("%x %X.%f").unwrap(), file!(), line!(), format_args!($($arg)+)));
    }
}


///Multiple-drop macro.
///
///It accepts any identifiers so be careful to pass right one.
#[macro_export]
macro_rules! drop {
    ($($arg:ident),+) => {
        $(drop($arg);)+
    };
}
