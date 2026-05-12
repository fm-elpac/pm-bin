// include this code in final bin crate
use pm_bin::log::debug;

// 编译信息
mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

/// version string (no debug)
pub fn get_version() -> String {
    let name = env!("CARGO_PKG_NAME");
    let v = env!("CARGO_PKG_VERSION");
    let target = built_info::TARGET;
    let features = built_info::FEATURES_LOWERCASE_STR;
    format!("{} version {} ({}, {})", name, v, target, features)
}

/// debug version string
pub fn get_version_debug() -> String {
    let git = env!("VERGEN_GIT_DESCRIBE");
    let profile = built_info::PROFILE;
    let time = env!("VERGEN_BUILD_TIMESTAMP");
    let rustc = built_info::RUSTC_VERSION;
    format!("{} {} {}, {}", git, profile, time, rustc)
}

/// show version info
pub fn print_version() {
    println!("{}", get_version());

    // debug
    debug!("{}", get_version_debug());
}
