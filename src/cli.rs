//! parse CLI arguments, print version info.
use std::env;

use env_logger;

/// init default env_logger
pub fn init_env_logger() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
}

/// simple CLI argument parse, process --version
pub fn cli_arg(print_version: fn() -> ()) -> Option<Vec<String>> {
    let a: Vec<String> = env::args().skip(1).collect();
    if a.len() > 0 {
        // 检查第 1 个参数
        match a[0].as_str() {
            "--version" | "--版本" => {
                print_version();
                return None;
            }
            _ => {}
        }
    }
    Some(a)
}

/// use this in final bin crate, this will include `pm_bin.rs`, which provide `print_version` function
#[macro_export]
macro_rules! pm_init {
    () => {
        mod pm_bin_include {
            include!(concat!(env!("OUT_DIR"), "/pm_bin.rs"));
        }
        use pm_bin_include::print_version;
    };
}
