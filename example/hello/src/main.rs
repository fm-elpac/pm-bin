use pm_bin::{cli_arg, init_env_logger, pm_init};

pm_init!();

fn main() {
    init_env_logger();

    if let Some(a) = cli_arg(print_version) {
        println!("{:?}", a);
    }
}
