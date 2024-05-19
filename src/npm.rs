use std::collections::HashMap;
use semver::Version;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct PackageMetadata {
    #[serde(rename(deserialize = "dist-tags"))]
    dist_tags: HashMap<String, String>
}

/// Gets the latest version of a package from the NPM Registry.
pub fn get_package_latest_version(package: String) -> Version {
    let req = reqwest::blocking::get(
        format!("https://registry.npmjs.org/{package}")
    )
        .unwrap()
        .json::<PackageMetadata>()
        .unwrap();

    Version::parse(req.dist_tags.get("latest").unwrap())
        .expect("Failed to parse dist-tags from remote registry.")
}
