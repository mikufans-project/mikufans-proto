// This file is @generated by prost-build.
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct AccountInfo {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
}
impl ::prost::Name for AccountInfo {
    const NAME: &'static str = "AccountInfo";
    const PACKAGE: &'static str = "bilibili.app.click.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.click.v1.AccountInfo".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.click.v1.AccountInfo".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppInfo {
    ///
    #[prost(string, tag = "1")]
    pub top_page_class: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub ftime: i64,
    ///
    #[prost(string, tag = "3")]
    pub did: ::prost::alloc::string::String,
}
impl ::prost::Name for AppInfo {
    const NAME: &'static str = "AppInfo";
    const PACKAGE: &'static str = "bilibili.app.click.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.click.v1.AppInfo".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.click.v1.AppInfo".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Extra {
    ///
    #[prost(string, tag = "1")]
    pub session: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub refer: ::prost::alloc::string::String,
}
impl ::prost::Name for Extra {
    const NAME: &'static str = "Extra";
    const PACKAGE: &'static str = "bilibili.app.click.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.click.v1.Extra".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.click.v1.Extra".into()
    }
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct HeartBeatReply {}
impl ::prost::Name for HeartBeatReply {
    const NAME: &'static str = "HeartBeatReply";
    const PACKAGE: &'static str = "bilibili.app.click.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.click.v1.HeartBeatReply".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.click.v1.HeartBeatReply".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeartBeatReq {
    ///
    #[prost(string, tag = "1")]
    pub session_v2: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "Stage", tag = "2")]
    pub stage: i32,
    ///
    #[prost(int64, tag = "3")]
    pub stream_timeout: i64,
    ///
    #[prost(int64, tag = "4")]
    pub batch_frequency: i64,
    ///
    #[prost(float, tag = "5")]
    pub frequency: f32,
    ///
    #[prost(message, optional, tag = "6")]
    pub video_meta: ::core::option::Option<VideoMeta>,
    ///
    #[prost(message, optional, tag = "7")]
    pub app_info: ::core::option::Option<AppInfo>,
    ///
    #[prost(message, optional, tag = "8")]
    pub account_info: ::core::option::Option<AccountInfo>,
    ///
    #[prost(message, optional, tag = "9")]
    pub pre_process_result: ::core::option::Option<PreProcessResult>,
    ///
    #[prost(message, repeated, tag = "10")]
    pub player_status: ::prost::alloc::vec::Vec<PlayerStatus>,
    ///
    #[prost(message, optional, tag = "11")]
    pub video_info: ::core::option::Option<VideoInfo>,
    ///
    #[prost(message, optional, tag = "12")]
    pub extra: ::core::option::Option<Extra>,
}
impl ::prost::Name for HeartBeatReq {
    const NAME: &'static str = "HeartBeatReq";
    const PACKAGE: &'static str = "bilibili.app.click.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.click.v1.HeartBeatReq".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.click.v1.HeartBeatReq".into()
    }
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PlayerStatus {
    ///
    #[prost(float, tag = "1")]
    pub playback_rate: f32,
    ///
    #[prost(int64, tag = "2")]
    pub progress: i64,
    ///
    #[prost(enumeration = "PlayState", tag = "3")]
    pub state: i32,
    ///
    #[prost(bool, tag = "4")]
    pub is_buffering: bool,
}
impl ::prost::Name for PlayerStatus {
    const NAME: &'static str = "PlayerStatus";
    const PACKAGE: &'static str = "bilibili.app.click.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.click.v1.PlayerStatus".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.click.v1.PlayerStatus".into()
    }
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PreProcessResult {
    ///
    #[prost(int64, tag = "1")]
    pub vt: i64,
}
impl ::prost::Name for PreProcessResult {
    const NAME: &'static str = "PreProcessResult";
    const PACKAGE: &'static str = "bilibili.app.click.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.click.v1.PreProcessResult".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.click.v1.PreProcessResult".into()
    }
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct VideoInfo {
    ///
    #[prost(int64, tag = "1")]
    pub cid_duration: i64,
}
impl ::prost::Name for VideoInfo {
    const NAME: &'static str = "VideoInfo";
    const PACKAGE: &'static str = "bilibili.app.click.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.click.v1.VideoInfo".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.click.v1.VideoInfo".into()
    }
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct VideoMeta {
    ///
    #[prost(int64, tag = "1")]
    pub aid: i64,
    ///
    #[prost(int64, tag = "2")]
    pub cid: i64,
}
impl ::prost::Name for VideoMeta {
    const NAME: &'static str = "VideoMeta";
    const PACKAGE: &'static str = "bilibili.app.click.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.click.v1.VideoMeta".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.click.v1.VideoMeta".into()
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PlayState {
    ///
    StateUnknown = 0,
    ///
    Preparing = 1,
    ///
    Prepared = 2,
    ///
    Playing = 3,
    ///
    Paused = 4,
    ///
    Stopped = 5,
    ///
    Failed = 6,
}
impl PlayState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::StateUnknown => "STATE_UNKNOWN",
            Self::Preparing => "PREPARING",
            Self::Prepared => "PREPARED",
            Self::Playing => "PLAYING",
            Self::Paused => "PAUSED",
            Self::Stopped => "STOPPED",
            Self::Failed => "FAILED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STATE_UNKNOWN" => Some(Self::StateUnknown),
            "PREPARING" => Some(Self::Preparing),
            "PREPARED" => Some(Self::Prepared),
            "PLAYING" => Some(Self::Playing),
            "PAUSED" => Some(Self::Paused),
            "STOPPED" => Some(Self::Stopped),
            "FAILED" => Some(Self::Failed),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Stage {
    ///
    Unknown = 0,
    ///
    Start = 1,
    ///
    End = 2,
    ///
    Sample = 3,
}
impl Stage {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unknown => "STAGE_UNKNOWN",
            Self::Start => "START",
            Self::End => "END",
            Self::Sample => "SAMPLE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STAGE_UNKNOWN" => Some(Self::Unknown),
            "START" => Some(Self::Start),
            "END" => Some(Self::End),
            "SAMPLE" => Some(Self::Sample),
            _ => None,
        }
    }
}
/// Generated client implementations.
#[cfg(feature = "feat-enable-generated-client")]
pub mod click_client {
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
    pub struct ClickClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ClickClient<T>
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
        ) -> ClickClient<InterceptedService<T, F>>
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
            ClickClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn heart_beat(
            &mut self,
            request: impl tonic::IntoRequest<super::HeartBeatReq>,
        ) -> std::result::Result<tonic::Response<super::HeartBeatReply>, tonic::Status> {
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
                "/bilibili.app.click.v1.Click/HeartBeat",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.click.v1.Click", "HeartBeat"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "feat-enable-generated-server")]
pub mod click_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ClickServer.
    #[async_trait]
    pub trait Click: std::marker::Send + std::marker::Sync + 'static {
        ///
        async fn heart_beat(
            &self,
            request: tonic::Request<super::HeartBeatReq>,
        ) -> std::result::Result<tonic::Response<super::HeartBeatReply>, tonic::Status>;
    }
    ///
    #[derive(Debug)]
    pub struct ClickServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> ClickServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ClickServer<T>
    where
        T: Click,
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
                "/bilibili.app.click.v1.Click/HeartBeat" => {
                    #[allow(non_camel_case_types)]
                    struct HeartBeatSvc<T: Click>(pub Arc<T>);
                    impl<T: Click> tonic::server::UnaryService<super::HeartBeatReq>
                    for HeartBeatSvc<T> {
                        type Response = super::HeartBeatReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HeartBeatReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Click>::heart_beat(&inner, request).await
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
                        let method = HeartBeatSvc(inner);
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
    impl<T> Clone for ClickServer<T> {
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
    pub const SERVICE_NAME: &str = "bilibili.app.click.v1.Click";
    impl<T> tonic::server::NamedService for ClickServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
