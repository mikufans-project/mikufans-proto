# Mikufans-proto

## Semver explanation

This crate may not not respect semver rules.

Take `8.24.3+build.16859280` as an example.

- `8` is the major version, which is not changed by this crate.
- `24` is the minor version, which is not changed by this crate.
- `3` is the patch version.

  The official version code seldom change this.
  
  However, for `innerVer` like `8240300`, the content may change.
  So we have to take `03` as the patch version but not `0`.

  Another example is `8241300`, the official semver is `8.24.1`, however
  we have to set the patch version to `13`.

- `build.16859280` is the build version, which is not changed by this crate.
