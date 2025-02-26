// This file is @generated by prost-build.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Device {
    ///
    #[prost(int32, tag = "1")]
    pub app_id: i32,
    ///
    #[prost(int32, tag = "2")]
    pub build: i32,
    ///
    #[prost(string, tag = "3")]
    pub buvid: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub mobi_app: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub platform: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub device: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub channel: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub brand: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub model: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub osver: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "11")]
    pub fp_local: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "12")]
    pub fp_remote: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "13")]
    pub version_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "14")]
    pub fp: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "15")]
    pub fts: i64,
    ///
    #[prost(string, tag = "16")]
    pub guest_id: ::prost::alloc::string::String,
}
impl ::prost::Name for Device {
    const NAME: &'static str = "Device";
    const PACKAGE: &'static str = "bilibili.metadata.device";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.metadata.device.Device".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.metadata.device.Device".into()
    }
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct DeviceType {}
impl ::prost::Name for DeviceType {
    const NAME: &'static str = "DeviceType";
    const PACKAGE: &'static str = "bilibili.metadata.device";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.metadata.device.DeviceType".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.metadata.device.DeviceType".into()
    }
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct MobiApp {}
impl ::prost::Name for MobiApp {
    const NAME: &'static str = "MobiApp";
    const PACKAGE: &'static str = "bilibili.metadata.device";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.metadata.device.MobiApp".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.metadata.device.MobiApp".into()
    }
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Platform {}
impl ::prost::Name for Platform {
    const NAME: &'static str = "Platform";
    const PACKAGE: &'static str = "bilibili.metadata.device";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.metadata.device.Platform".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.metadata.device.Platform".into()
    }
}
