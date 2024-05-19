# unused-pkg

This tool is used to check for unused packages in `rust` and `javascript` 
projects.

**I know there are only tools to check for unused packages, but this is a side 
project.**

## How to use it

Simply call it in the directory containing the `package.json` or `Cargo.toml`
file. It will list all unused and outdated packages.

## Roadmap
- [X] Support for Cargo.toml unused crates + outdated crates listing.
    - [ ] Workspace support
- [X] Support for package.json unused dependencies + outdated dependencies listing.
- [ ] Concurrent listing.
- [ ] Do not consider files outside the module tree.
