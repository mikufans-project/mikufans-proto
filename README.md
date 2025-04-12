# Mikufans-proto

[![crates.io](https://img.shields.io/crates/v/mikufans-proto)](https://crates.io/crates/mikufans-proto)
[![docs.rs](https://img.shields.io/docsrs/mikufans-proto)](https://docs.rs/crate/mikufans-proto/latest)
[![github tags](https://img.shields.io/github/v/tag/mikufans-project/mikufans-proto)](https://github.com/mikufans-project/mikufans-proto/tags)

## Semver explanation

Starting from 8.41.0, we will change the naming strategy.

Take `8.41.0-rc.1+build.18365411` as an example.

- `8` is the major version, which is not changed by this crate.
- `41` is the minor version, which is not changed by this crate.
- `0` is the crate patch version, which is not changed by this crate.
- `rc.1` is the pre-release version.

  During the entire life cycle of the current version (e.g., `8.41.0`),
  if upstream code is updated or this repository makes certain changes,
  we will increment the pre-release version until the official release
  of the next version, such as `8.42.0` (or `8.41.1`, though this is less common).
  Only then will we release the stable version of the current version.

  For breaking changes within this repository itself, we will follow semver rules
  and be shipped at the next minor version release.

- `build.18365411` is the actual build version, which is not changed by this crate.
