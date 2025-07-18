// This file is @generated by prost-build.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AiAudioItem {
    ///
    #[prost(message, repeated, tag = "1")]
    pub audio_info: ::prost::alloc::vec::Vec<DashItem>,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub button_title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub subtitle_lang: ::prost::alloc::string::String,
}
impl ::prost::Name for AiAudioItem {
    const NAME: &'static str = "AIAudioItem";
    const PACKAGE: &'static str = "bilibili.app.story.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.story.v1.AIAudioItem".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.story.v1.AIAudioItem".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AiAudioReply {
    ///
    #[prost(bool, tag = "1")]
    pub support_ai_audio: bool,
    ///
    #[prost(message, repeated, tag = "2")]
    pub ai_audio_items: ::prost::alloc::vec::Vec<AiAudioItem>,
    ///
    #[prost(string, tag = "3")]
    pub ai_open_toast: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub ai_close_toast: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "5")]
    pub language: ::core::option::Option<Language>,
}
impl ::prost::Name for AiAudioReply {
    const NAME: &'static str = "AIAudioReply";
    const PACKAGE: &'static str = "bilibili.app.story.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.story.v1.AIAudioReply".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.story.v1.AIAudioReply".into()
    }
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct AiAudioReq {
    ///
    #[prost(int64, tag = "1")]
    pub aid: i64,
    ///
    #[prost(int64, tag = "2")]
    pub cid: i64,
    ///
    #[prost(int64, tag = "3")]
    pub up_mid: i64,
}
impl ::prost::Name for AiAudioReq {
    const NAME: &'static str = "AIAudioReq";
    const PACKAGE: &'static str = "bilibili.app.story.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.story.v1.AIAudioReq".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.story.v1.AIAudioReq".into()
    }
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ArchiveStatusReply {
    ///
    #[prost(bool, tag = "1")]
    pub is_archive_normal: bool,
}
impl ::prost::Name for ArchiveStatusReply {
    const NAME: &'static str = "ArchiveStatusReply";
    const PACKAGE: &'static str = "bilibili.app.story.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.story.v1.ArchiveStatusReply".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.story.v1.ArchiveStatusReply".into()
    }
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ArchiveStatusReq {
    ///
    #[prost(int64, tag = "1")]
    pub aid: i64,
}
impl ::prost::Name for ArchiveStatusReq {
    const NAME: &'static str = "ArchiveStatusReq";
    const PACKAGE: &'static str = "bilibili.app.story.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.story.v1.ArchiveStatusReq".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.story.v1.ArchiveStatusReq".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BgmPlayReply {
    ///
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
}
impl ::prost::Name for BgmPlayReply {
    const NAME: &'static str = "BgmPlayReply";
    const PACKAGE: &'static str = "bilibili.app.story.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.story.v1.BgmPlayReply".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.story.v1.BgmPlayReply".into()
    }
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct BgmPlayReq {
    ///
    #[prost(int64, tag = "1")]
    pub music_id: i64,
}
impl ::prost::Name for BgmPlayReq {
    const NAME: &'static str = "BgmPlayReq";
    const PACKAGE: &'static str = "bilibili.app.story.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.story.v1.BgmPlayReq".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.story.v1.BgmPlayReq".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DashItem {
    ///
    #[prost(int32, tag = "1")]
    pub id: i32,
    ///
    #[prost(string, tag = "2")]
    pub base_url: ::prost::alloc::string::String,
    ///
    #[prost(string, repeated, tag = "3")]
    pub backup_url: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(int32, tag = "4")]
    pub bandwidth: i32,
    ///
    #[prost(int32, tag = "5")]
    pub codecid: i32,
    ///
    #[prost(string, tag = "6")]
    pub md5: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "7")]
    pub size: i64,
    ///
    #[prost(string, tag = "8")]
    pub frame_rate: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub widevine_pssh: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub bilidrm_uri: ::prost::alloc::string::String,
}
impl ::prost::Name for DashItem {
    const NAME: &'static str = "DashItem";
    const PACKAGE: &'static str = "bilibili.app.story.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.story.v1.DashItem".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.story.v1.DashItem".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Language {
    ///
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<LanguageItem>,
}
impl ::prost::Name for Language {
    const NAME: &'static str = "Language";
    const PACKAGE: &'static str = "bilibili.app.story.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.story.v1.Language".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.story.v1.Language".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LanguageItem {
    ///
    #[prost(string, tag = "1")]
    pub lang: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub button_title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub subtitle_lang: ::prost::alloc::string::String,
}
impl ::prost::Name for LanguageItem {
    const NAME: &'static str = "LanguageItem";
    const PACKAGE: &'static str = "bilibili.app.story.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.story.v1.LanguageItem".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.story.v1.LanguageItem".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RerankAdInfo {
    ///
    #[prost(string, tag = "1")]
    pub request_id: ::prost::alloc::string::String,
}
impl ::prost::Name for RerankAdInfo {
    const NAME: &'static str = "RerankAdInfo";
    const PACKAGE: &'static str = "bilibili.app.story.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.story.v1.RerankAdInfo".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.story.v1.RerankAdInfo".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RerankCardInfo {
    ///
    #[prost(int32, tag = "1")]
    pub display_id: i32,
    ///
    #[prost(int32, tag = "2")]
    pub display_pos: i32,
    ///
    #[prost(string, tag = "3")]
    pub card_goto: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub param: ::prost::alloc::string::String,
    ///
    #[prost(map = "string, string", tag = "5")]
    pub player_args: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    ///
    #[prost(oneof = "rerank_card_info::Info", tags = "6")]
    pub info: ::core::option::Option<rerank_card_info::Info>,
}
/// Nested message and enum types in `RerankCardInfo`.
pub mod rerank_card_info {
    ///
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Info {
        ///
        #[prost(message, tag = "6")]
        AdInfo(super::RerankAdInfo),
    }
}
impl ::prost::Name for RerankCardInfo {
    const NAME: &'static str = "RerankCardInfo";
    const PACKAGE: &'static str = "bilibili.app.story.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.story.v1.RerankCardInfo".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.story.v1.RerankCardInfo".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RerankReply {
    ///
    #[prost(message, repeated, tag = "1")]
    pub card: ::prost::alloc::vec::Vec<RerankCardInfo>,
}
impl ::prost::Name for RerankReply {
    const NAME: &'static str = "RerankReply";
    const PACKAGE: &'static str = "bilibili.app.story.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.story.v1.RerankReply".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.story.v1.RerankReply".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RerankReq {
    ///
    #[prost(int64, tag = "1")]
    pub pre_ad_start_ts: i64,
    ///
    #[prost(int64, tag = "2")]
    pub pre_ad_end_ts: i64,
    ///
    #[prost(message, optional, tag = "3")]
    pub pre_ad_card: ::core::option::Option<RerankCardInfo>,
    ///
    #[prost(int64, tag = "4")]
    pub last_card_ts: i64,
    ///
    #[prost(message, optional, tag = "5")]
    pub last_card: ::core::option::Option<RerankCardInfo>,
    ///
    #[prost(message, repeated, tag = "6")]
    pub unexposed_card: ::prost::alloc::vec::Vec<RerankCardInfo>,
    ///
    #[prost(int32, tag = "7")]
    pub last_ad_gap: i32,
}
impl ::prost::Name for RerankReq {
    const NAME: &'static str = "RerankReq";
    const PACKAGE: &'static str = "bilibili.app.story.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.story.v1.RerankReq".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.story.v1.RerankReq".into()
    }
}
/// Generated client implementations.
#[cfg(feature = "feat-enable-generated-client")]
pub mod story_client {
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
    pub struct StoryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> StoryClient<T>
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
        ) -> StoryClient<InterceptedService<T, F>>
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
            StoryClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn ai_audio(
            &mut self,
            request: impl tonic::IntoRequest<super::AiAudioReq>,
        ) -> std::result::Result<tonic::Response<super::AiAudioReply>, tonic::Status> {
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
                "/bilibili.app.story.v1.Story/AIAudio",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.story.v1.Story", "AIAudio"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn archive_status(
            &mut self,
            request: impl tonic::IntoRequest<super::ArchiveStatusReq>,
        ) -> std::result::Result<
            tonic::Response<super::ArchiveStatusReply>,
            tonic::Status,
        > {
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
                "/bilibili.app.story.v1.Story/ArchiveStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.story.v1.Story", "ArchiveStatus"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn bgm_play(
            &mut self,
            request: impl tonic::IntoRequest<super::BgmPlayReq>,
        ) -> std::result::Result<tonic::Response<super::BgmPlayReply>, tonic::Status> {
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
                "/bilibili.app.story.v1.Story/BgmPlay",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.story.v1.Story", "BgmPlay"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn rerank(
            &mut self,
            request: impl tonic::IntoRequest<super::RerankReq>,
        ) -> std::result::Result<tonic::Response<super::RerankReply>, tonic::Status> {
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
                "/bilibili.app.story.v1.Story/Rerank",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.story.v1.Story", "Rerank"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "feat-enable-generated-server")]
pub mod story_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with StoryServer.
    #[async_trait]
    pub trait Story: std::marker::Send + std::marker::Sync + 'static {
        ///
        async fn ai_audio(
            &self,
            request: tonic::Request<super::AiAudioReq>,
        ) -> std::result::Result<tonic::Response<super::AiAudioReply>, tonic::Status>;
        ///
        async fn archive_status(
            &self,
            request: tonic::Request<super::ArchiveStatusReq>,
        ) -> std::result::Result<
            tonic::Response<super::ArchiveStatusReply>,
            tonic::Status,
        >;
        ///
        async fn bgm_play(
            &self,
            request: tonic::Request<super::BgmPlayReq>,
        ) -> std::result::Result<tonic::Response<super::BgmPlayReply>, tonic::Status>;
        ///
        async fn rerank(
            &self,
            request: tonic::Request<super::RerankReq>,
        ) -> std::result::Result<tonic::Response<super::RerankReply>, tonic::Status>;
    }
    ///
    #[derive(Debug)]
    pub struct StoryServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> StoryServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for StoryServer<T>
    where
        T: Story,
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
                "/bilibili.app.story.v1.Story/AIAudio" => {
                    #[allow(non_camel_case_types)]
                    struct AIAudioSvc<T: Story>(pub Arc<T>);
                    impl<T: Story> tonic::server::UnaryService<super::AiAudioReq>
                    for AIAudioSvc<T> {
                        type Response = super::AiAudioReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AiAudioReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Story>::ai_audio(&inner, request).await
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
                        let method = AIAudioSvc(inner);
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
                "/bilibili.app.story.v1.Story/ArchiveStatus" => {
                    #[allow(non_camel_case_types)]
                    struct ArchiveStatusSvc<T: Story>(pub Arc<T>);
                    impl<T: Story> tonic::server::UnaryService<super::ArchiveStatusReq>
                    for ArchiveStatusSvc<T> {
                        type Response = super::ArchiveStatusReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ArchiveStatusReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Story>::archive_status(&inner, request).await
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
                        let method = ArchiveStatusSvc(inner);
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
                "/bilibili.app.story.v1.Story/BgmPlay" => {
                    #[allow(non_camel_case_types)]
                    struct BgmPlaySvc<T: Story>(pub Arc<T>);
                    impl<T: Story> tonic::server::UnaryService<super::BgmPlayReq>
                    for BgmPlaySvc<T> {
                        type Response = super::BgmPlayReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BgmPlayReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Story>::bgm_play(&inner, request).await
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
                        let method = BgmPlaySvc(inner);
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
                "/bilibili.app.story.v1.Story/Rerank" => {
                    #[allow(non_camel_case_types)]
                    struct RerankSvc<T: Story>(pub Arc<T>);
                    impl<T: Story> tonic::server::UnaryService<super::RerankReq>
                    for RerankSvc<T> {
                        type Response = super::RerankReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RerankReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Story>::rerank(&inner, request).await
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
                        let method = RerankSvc(inner);
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
    impl<T> Clone for StoryServer<T> {
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
    pub const SERVICE_NAME: &str = "bilibili.app.story.v1.Story";
    impl<T> tonic::server::NamedService for StoryServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
