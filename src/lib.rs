pub mod datacenter {
    pub mod hakase {
        pub mod protobuf {
            include!("datacenter.hakase.protobuf.rs");
        }
    }
}

mod generated;

// re-export generated types
pub use generated::*;

/// The version code.
pub static TARGET_VERSION: &'static str = "8.32.0";

/// The build serial number.
pub static TARGET_BUILD_SN: i32 = 17584272;
