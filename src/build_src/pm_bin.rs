// include this code in final bin crate
use pm_bin::log::debug;

// 编译信息
mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

/// show version info
pub fn print_version() {
    let name = env!("CARGO_PKG_NAME");
    let v = env!("CARGO_PKG_VERSION");
    let target = built_info::TARGET;
    let features = built_info::FEATURES_LOWERCASE_STR;
    println!("{} version {} ({}, {})", name, v, target, features);

    // debug
    let git = env!("VERGEN_GIT_DESCRIBE");
    let profile = built_info::PROFILE;
    let time = env!("VERGEN_BUILD_TIMESTAMP");
    let rustc = built_info::RUSTC_VERSION;
    debug!("{} {} {}, {}", git, profile, time, rustc);
}
