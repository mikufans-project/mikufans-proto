#[allow(clippy::empty_docs)]
#[cfg(feature = "datacenter")]
pub mod datacenter {
    #[cfg(feature = "datacenter_hakase")]
    pub mod hakase {
        #[cfg(feature = "datacenter_hakase_protobuf")]
        pub mod protobuf {
            include!("datacenter.hakase.protobuf.rs");
        }
    }
}

#[allow(deprecated)]
#[allow(clippy::empty_docs)]
mod generated;

#[allow(unused_imports)]
// re-export generated types
pub use generated::*;

/// The version code.
pub const TARGET_VERSION: &str = "8.49.0";

/// The version code number.
pub const TARGET_VERSION_CODE: i32 = 8490300;

/// The build serial number.
pub const TARGET_BUILD_SN: i32 = 19246369;
