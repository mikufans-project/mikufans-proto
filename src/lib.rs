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
