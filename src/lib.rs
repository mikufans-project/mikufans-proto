#[allow(clippy::empty_docs)]
pub mod datacenter {
    pub mod hakase {
        pub mod protobuf {
            include!("datacenter.hakase.protobuf.rs");
        }
    }
}

#[allow(deprecated)]
#[allow(clippy::empty_docs)]
mod generated;

// re-export generated types
pub use generated::*;

/// The version code.
pub static TARGET_VERSION: &str = "8.36.0";

/// The build serial number.
pub static TARGET_BUILD_SN: i32 = 17907690;
