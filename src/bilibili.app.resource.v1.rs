// This file is @generated by prost-build.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Download {
    ///
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub extra_value: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "3")]
    pub list: ::prost::alloc::vec::Vec<ResourceItem>,
}
impl ::prost::Name for Download {
    const NAME: &'static str = "Download";
    const PACKAGE: &'static str = "bilibili.app.resource.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.resource.v1.Download".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.resource.v1.Download".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadReply {
    ///
    #[prost(string, tag = "1")]
    pub ver: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "2")]
    pub resource: ::prost::alloc::vec::Vec<Download>,
    ///
    #[prost(map = "string, message", tag = "3")]
    pub dwtime: ::std::collections::HashMap<::prost::alloc::string::String, DwTime>,
}
impl ::prost::Name for DownloadReply {
    const NAME: &'static str = "DownloadReply";
    const PACKAGE: &'static str = "bilibili.app.resource.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.resource.v1.DownloadReply".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.resource.v1.DownloadReply".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadReq {
    ///
    #[prost(string, tag = "1")]
    pub ver: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub screen_width: i64,
    ///
    #[prost(int64, tag = "4")]
    pub screen_height: i64,
}
impl ::prost::Name for DownloadReq {
    const NAME: &'static str = "DownloadReq";
    const PACKAGE: &'static str = "bilibili.app.resource.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.resource.v1.DownloadReq".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.resource.v1.DownloadReq".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DwTime {
    ///
    #[prost(int32, tag = "1")]
    pub r#type: i32,
    ///
    #[prost(message, repeated, tag = "2")]
    pub peak: ::prost::alloc::vec::Vec<DwTimePiece>,
    ///
    #[prost(message, repeated, tag = "3")]
    pub low: ::prost::alloc::vec::Vec<DwTimePiece>,
}
impl ::prost::Name for DwTime {
    const NAME: &'static str = "DwTime";
    const PACKAGE: &'static str = "bilibili.app.resource.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.resource.v1.DwTime".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.resource.v1.DwTime".into()
    }
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct DwTimePiece {
    ///
    #[prost(int64, tag = "1")]
    pub start: i64,
    ///
    #[prost(int64, tag = "2")]
    pub end: i64,
}
impl ::prost::Name for DwTimePiece {
    const NAME: &'static str = "DwTimePiece";
    const PACKAGE: &'static str = "bilibili.app.resource.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.resource.v1.DwTimePiece".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.resource.v1.DwTimePiece".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Host {
    ///
    #[prost(string, tag = "1")]
    pub boss: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub bfs: ::prost::alloc::string::String,
}
impl ::prost::Name for Host {
    const NAME: &'static str = "Host";
    const PACKAGE: &'static str = "bilibili.app.resource.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.resource.v1.Host".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.resource.v1.Host".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReply {
    ///
    #[prost(string, tag = "1")]
    pub env: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "2")]
    pub pools: ::prost::alloc::vec::Vec<PoolReply>,
    ///
    #[prost(int64, tag = "3")]
    pub list_version: i64,
    ///
    #[prost(message, optional, tag = "4")]
    pub host: ::core::option::Option<Host>,
}
impl ::prost::Name for ListReply {
    const NAME: &'static str = "ListReply";
    const PACKAGE: &'static str = "bilibili.app.resource.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.resource.v1.ListReply".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.resource.v1.ListReply".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReq {
    ///
    #[prost(string, tag = "1")]
    pub pool_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub module_name: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "3")]
    pub version_list: ::prost::alloc::vec::Vec<VersionListReq>,
    ///
    #[prost(enumeration = "EnvType", tag = "4")]
    pub env: i32,
    ///
    #[prost(int32, tag = "5")]
    pub sys_ver: i32,
    ///
    #[prost(int32, tag = "6")]
    pub scale: i32,
    ///
    #[prost(int32, tag = "7")]
    pub arch: i32,
    ///
    #[prost(int64, tag = "8")]
    pub list_version: i64,
    ///
    #[prost(int32, tag = "9")]
    pub lite: i32,
    ///
    #[prost(int64, tag = "10")]
    pub support_type: i64,
    ///
    #[prost(bool, tag = "11")]
    pub support_unzip_password: bool,
    ///
    #[prost(bool, tag = "12")]
    pub support_peak: bool,
}
impl ::prost::Name for ListReq {
    const NAME: &'static str = "ListReq";
    const PACKAGE: &'static str = "bilibili.app.resource.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.resource.v1.ListReq".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.resource.v1.ListReq".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleReply {
    ///
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub version: i64,
    ///
    #[prost(string, tag = "3")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub md5: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub total_md5: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "IncrementType", tag = "6")]
    pub increment: i32,
    ///
    #[prost(bool, tag = "7")]
    pub is_wifi: bool,
    ///
    #[prost(enumeration = "LevelType", tag = "8")]
    pub level: i32,
    ///
    #[prost(string, tag = "9")]
    pub filename: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub file_type: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "11")]
    pub file_size: i64,
    ///
    #[prost(enumeration = "CompressType", tag = "12")]
    pub compress: i32,
    ///
    #[prost(int64, tag = "13")]
    pub publish_time: i64,
    ///
    #[prost(int64, tag = "14")]
    pub pool_id: i64,
    ///
    #[prost(int64, tag = "15")]
    pub module_id: i64,
    ///
    #[prost(int64, tag = "16")]
    pub version_id: i64,
    ///
    #[prost(int64, tag = "17")]
    pub file_id: i64,
    ///
    #[prost(bool, tag = "18")]
    pub zip_check: bool,
    ///
    #[prost(int64, tag = "19")]
    pub download_strategy: i64,
    ///
    #[prost(int64, tag = "20")]
    pub experiment_strategy: i64,
    ///
    #[prost(map = "string, message", tag = "21")]
    pub patch_map: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        PatchInfo,
    >,
    ///
    #[prost(int64, tag = "22")]
    pub support_type: i64,
    ///
    #[prost(bool, tag = "23")]
    pub password_required: bool,
    ///
    #[prost(string, tag = "24")]
    pub password: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "25")]
    pub weight: i64,
}
impl ::prost::Name for ModuleReply {
    const NAME: &'static str = "ModuleReply";
    const PACKAGE: &'static str = "bilibili.app.resource.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.resource.v1.ModuleReply".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.resource.v1.ModuleReply".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchInfo {
    ///
    #[prost(string, tag = "1")]
    pub md5: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub size: i64,
    ///
    #[prost(int64, tag = "4")]
    pub source_version: i64,
}
impl ::prost::Name for PatchInfo {
    const NAME: &'static str = "PatchInfo";
    const PACKAGE: &'static str = "bilibili.app.resource.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.resource.v1.PatchInfo".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.resource.v1.PatchInfo".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolReply {
    ///
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "2")]
    pub modules: ::prost::alloc::vec::Vec<ModuleReply>,
}
impl ::prost::Name for PoolReply {
    const NAME: &'static str = "PoolReply";
    const PACKAGE: &'static str = "bilibili.app.resource.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.resource.v1.PoolReply".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.resource.v1.PoolReply".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceItem {
    ///
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub file_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub r#type: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub hash: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "6")]
    pub size: i32,
    ///
    #[prost(int32, tag = "7")]
    pub expect_dw: i32,
    ///
    #[prost(int64, tag = "8")]
    pub effect_time: i64,
    ///
    #[prost(int64, tag = "9")]
    pub expire_time: i64,
    ///
    #[prost(int32, tag = "10")]
    pub priority: i32,
    ///
    #[prost(string, tag = "11")]
    pub extra: ::prost::alloc::string::String,
}
impl ::prost::Name for ResourceItem {
    const NAME: &'static str = "ResourceItem";
    const PACKAGE: &'static str = "bilibili.app.resource.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.resource.v1.ResourceItem".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.resource.v1.ResourceItem".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionListReq {
    ///
    #[prost(string, tag = "1")]
    pub pool_name: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "2")]
    pub versions: ::prost::alloc::vec::Vec<VersionReq>,
}
impl ::prost::Name for VersionListReq {
    const NAME: &'static str = "VersionListReq";
    const PACKAGE: &'static str = "bilibili.app.resource.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.resource.v1.VersionListReq".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.resource.v1.VersionListReq".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionReq {
    ///
    #[prost(string, tag = "1")]
    pub module_name: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub version: i64,
    ///
    #[prost(int64, tag = "3")]
    pub r#type: i64,
}
impl ::prost::Name for VersionReq {
    const NAME: &'static str = "VersionReq";
    const PACKAGE: &'static str = "bilibili.app.resource.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.resource.v1.VersionReq".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.resource.v1.VersionReq".into()
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CompressType {
    ///
    Unzip = 0,
    ///
    Original = 1,
}
impl CompressType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unzip => "Unzip",
            Self::Original => "Original",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Unzip" => Some(Self::Unzip),
            "Original" => Some(Self::Original),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EnvType {
    ///
    Unknown = 0,
    ///
    Release = 1,
    ///
    Test = 2,
}
impl EnvType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unknown => "Unknown",
            Self::Release => "Release",
            Self::Test => "Test",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Unknown" => Some(Self::Unknown),
            "Release" => Some(Self::Release),
            "Test" => Some(Self::Test),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IncrementType {
    ///
    Total = 0,
    ///
    Incremental = 1,
}
impl IncrementType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Total => "Total",
            Self::Incremental => "Incremental",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Total" => Some(Self::Total),
            "Incremental" => Some(Self::Incremental),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LevelType {
    ///
    Undefined = 0,
    ///
    High = 1,
    ///
    Middle = 2,
    ///
    Low = 3,
}
impl LevelType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Undefined => "Undefined",
            Self::High => "High",
            Self::Middle => "Middle",
            Self::Low => "Low",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Undefined" => Some(Self::Undefined),
            "High" => Some(Self::High),
            "Middle" => Some(Self::Middle),
            "Low" => Some(Self::Low),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod legacy_resource_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    #[derive(Debug, Clone)]
    pub struct LegacyResourceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> LegacyResourceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> LegacyResourceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            LegacyResourceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        ///
        pub async fn download(
            &mut self,
            request: impl tonic::IntoRequest<super::DownloadReq>,
        ) -> std::result::Result<tonic::Response<super::DownloadReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.app.resource.v1.LegacyResource/Download",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.resource.v1.LegacyResource",
                        "Download",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod module_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    #[derive(Debug, Clone)]
    pub struct ModuleClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ModuleClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ModuleClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            ModuleClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        ///
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::ListReq>,
        ) -> std::result::Result<tonic::Response<super::ListReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.app.resource.v1.Module/List",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.resource.v1.Module", "List"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod legacy_resource_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with LegacyResourceServer.
    #[async_trait]
    pub trait LegacyResource: std::marker::Send + std::marker::Sync + 'static {
        ///
        async fn download(
            &self,
            request: tonic::Request<super::DownloadReq>,
        ) -> std::result::Result<tonic::Response<super::DownloadReply>, tonic::Status>;
    }
    ///
    #[derive(Debug)]
    pub struct LegacyResourceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> LegacyResourceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for LegacyResourceServer<T>
    where
        T: LegacyResource,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/bilibili.app.resource.v1.LegacyResource/Download" => {
                    #[allow(non_camel_case_types)]
                    struct DownloadSvc<T: LegacyResource>(pub Arc<T>);
                    impl<
                        T: LegacyResource,
                    > tonic::server::UnaryService<super::DownloadReq>
                    for DownloadSvc<T> {
                        type Response = super::DownloadReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DownloadReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as LegacyResource>::download(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DownloadSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for LegacyResourceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "bilibili.app.resource.v1.LegacyResource";
    impl<T> tonic::server::NamedService for LegacyResourceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated server implementations.
pub mod module_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ModuleServer.
    #[async_trait]
    pub trait Module: std::marker::Send + std::marker::Sync + 'static {
        ///
        async fn list(
            &self,
            request: tonic::Request<super::ListReq>,
        ) -> std::result::Result<tonic::Response<super::ListReply>, tonic::Status>;
    }
    ///
    #[derive(Debug)]
    pub struct ModuleServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> ModuleServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ModuleServer<T>
    where
        T: Module,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/bilibili.app.resource.v1.Module/List" => {
                    #[allow(non_camel_case_types)]
                    struct ListSvc<T: Module>(pub Arc<T>);
                    impl<T: Module> tonic::server::UnaryService<super::ListReq>
                    for ListSvc<T> {
                        type Response = super::ListReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Module>::list(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for ModuleServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "bilibili.app.resource.v1.Module";
    impl<T> tonic::server::NamedService for ModuleServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
