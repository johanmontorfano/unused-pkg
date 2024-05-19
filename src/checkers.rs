use std::{collections::HashSet, fs::read_to_string, path::PathBuf, time::Duration};
use cargo_toml::{Manifest, Dependency::{Simple, Detailed}};
use crates_io_api::SyncClient;
use semver::Version;
use crate::{fs_utils::recursive_listing, info, npm::get_package_latest_version, warn};
use package_json_schema::PackageJson;

pub fn check_cargo_toml(cwd: &mut PathBuf) {
    let mut cargo_toml_path = cwd.clone();
    let mut src_dir_path = cwd.clone();

    cargo_toml_path.push("Cargo.toml");
    src_dir_path.push("src/");
    
    let manifest = Manifest::from_path(cargo_toml_path)
        .expect("Failed to read Cargo.toml");
    let client = SyncClient::new(
        "unused-pkg (me@johanmontorfano.com)", 
        Duration::from_millis(1000)
    ).unwrap();
    let mut found_dependencies_in_src_files = HashSet::new();

    recursive_listing(&src_dir_path).iter().for_each(|a| { 
        manifest.dependencies.iter().for_each(|(dependency, _)| {
            if a.find(dependency).is_some() {
                found_dependencies_in_src_files.insert(dependency);
            }
        });
    });

    manifest.dependencies.iter().for_each(|(dependency, details)| {
        let version = match details {
            Simple(version) => Some(version.clone()),
            Detailed(data) => data.version.clone(),
            _ => Some("X".to_string())
        }.unwrap();
        let v_semver = Version::parse(&version).unwrap();
    
        if found_dependencies_in_src_files.contains(dependency) == false {
            print!("\x1b[90m");
        }
        print!("{dependency}: {version}");
        print!("\x1b[0m");
        if details.is_crates_io() {
            let crate_data = client.get_crate(dependency).unwrap();
            let remote_version = crate_data.versions.first().unwrap();
            let r_semver = Version::parse(&remote_version.num).unwrap();
            if v_semver < r_semver {
                warn!(" -> {}", remote_version.num);
            } else {
                print!("\n");
            }
        } else {
            info!("[Can't check for new versions: non crates.io source]");
        }
    });
}

pub fn check_pkg_json(cwd: &PathBuf) {
    let mut package_json_path = cwd.clone();
    package_json_path.push("package.json");

    let manifest_content = read_to_string(package_json_path).unwrap();
    let manifest = PackageJson::try_from(manifest_content).unwrap();
    let dependencies = manifest.dependencies.expect("No dependency declared.");
    let mut found_dependencies_in_src_files = HashSet::new();

    recursive_listing(cwd).iter().for_each(|a| {
        dependencies.iter().for_each(|d| {
            if a.contains(d.0) {
                found_dependencies_in_src_files.insert(d.0);
            }
        })
    });

    dependencies.iter().for_each(|(dependency, version)| {
        let latest_version = get_package_latest_version(dependency.into());
        let v_semver = Version::parse(&version.replace("^", "")).unwrap();
        if found_dependencies_in_src_files.contains(dependency) == false {
            print!("\x1b[90m");
        }
        print!("{dependency}: {version}\x1b[0m");
        if v_semver < latest_version {
            warn!(" -> {latest_version}");
        } else {
            print!("\n");
        }
    });

}
