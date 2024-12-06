# Mikufans-proto

![Crates.io Version](https://img.shields.io/crates/v/mikufans-proto)
![GitHub Tag](https://img.shields.io/github/v/tag/cxw620/mikufans-proto)
![Crates.io Total Downloads](https://img.shields.io/crates/d/mikufans-proto)

## Semver explanation

**TL, DR**
We STRONGLY recommend you to specify the exact version like `mikufans-proto = "=8.24.3"`
in your `Cargo.toml` to avoid unexpected breaking changes.

---

This crate may not not respect semver rules.

Take `8.24.3+build.16859280` as an example.

The official version is actually `8.24.0`, with *innerVer* `8240300`.

- `8` is the major version, which is not changed by this crate.
- `24` is the minor version, which is not changed by this crate.
- `3` is the crate patch version.

  Though patch version does not change, there may still be some changes
  in the content comparing with `8240300`, so we have to take `03` as
  the crate patch version but not `0`.

  One more example is `8.24.1`, with *innerVer* `8241300`. The corresponding
  patch version will be `13`.

- `build.16859280` is the build version, which is not changed by this crate.
