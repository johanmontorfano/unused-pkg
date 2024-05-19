mod npm;
mod fs_utils;
mod checkers;
mod log_macro;
use std::{env, path::PathBuf, str::FromStr};
use fs_utils::try_exists_at_path;
use checkers::{check_cargo_toml, check_pkg_json};

fn main() {
    let mut cwd = if env::args().len() > 1 {
        PathBuf::from_str(env::args().collect::<Vec<String>>().get(1).unwrap())
            .expect("Failed to get provided directory.")
    } else {
        env::current_dir().expect("Failed to get current dir.")
    };
    let has_cargo = try_exists_at_path(&cwd, "Cargo.toml");
    let has_json_pkg = try_exists_at_path(&cwd, "package.json");

    if has_cargo {
        info!("Cargo.toml found.");
        check_cargo_toml(&mut cwd);
    }
    else if has_json_pkg {
        info!("package.json found.");
        check_pkg_json(&mut cwd);
    }
    else {
        error!("No manifest found.");
    }
}


