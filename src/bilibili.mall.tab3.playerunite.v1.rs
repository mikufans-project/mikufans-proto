// This file is @generated by prost-build.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayViewUniteReply {
    ///
    #[prost(message, optional, tag = "1")]
    pub vod_info: ::core::option::Option<
        super::super::super::super::playershared::VodInfo,
    >,
    ///
    #[prost(message, optional, tag = "2")]
    pub play_arc_conf: ::core::option::Option<
        super::super::super::super::playershared::PlayArcConf,
    >,
    ///
    #[prost(message, optional, tag = "3")]
    pub play_device_conf: ::core::option::Option<
        super::super::super::super::playershared::PlayDeviceConf,
    >,
    ///
    #[prost(message, optional, tag = "4")]
    pub event: ::core::option::Option<super::super::super::super::playershared::Event>,
    ///
    #[prost(message, optional, tag = "5")]
    pub supplement: ::core::option::Option<::prost_types::Any>,
    ///
    #[prost(message, optional, tag = "6")]
    pub play_arc: ::core::option::Option<
        super::super::super::super::playershared::PlayArc,
    >,
    ///
    #[prost(message, optional, tag = "7")]
    pub qn_trial_info: ::core::option::Option<
        super::super::super::super::playershared::QnTrialInfo,
    >,
    ///
    #[prost(message, optional, tag = "8")]
    pub history: ::core::option::Option<
        super::super::super::super::playershared::History,
    >,
    ///
    #[prost(message, optional, tag = "9")]
    pub view_info: ::core::option::Option<
        super::super::super::super::playershared::ViewInfo,
    >,
}
impl ::prost::Name for PlayViewUniteReply {
    const NAME: &'static str = "PlayViewUniteReply";
    const PACKAGE: &'static str = "bilibili.mall.tab3.playerunite.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.mall.tab3.playerunite.v1.PlayViewUniteReply".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.mall.tab3.playerunite.v1.PlayViewUniteReply".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayViewUniteReq {
    ///
    #[prost(message, optional, tag = "1")]
    pub vod: ::core::option::Option<super::super::super::super::playershared::VideoVod>,
    ///
    #[prost(string, tag = "2")]
    pub spmid: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub from_spmid: ::prost::alloc::string::String,
    ///
    #[prost(map = "string, string", tag = "4")]
    pub extra_content: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    ///
    #[prost(string, tag = "5")]
    pub bvid: ::prost::alloc::string::String,
}
impl ::prost::Name for PlayViewUniteReq {
    const NAME: &'static str = "PlayViewUniteReq";
    const PACKAGE: &'static str = "bilibili.mall.tab3.playerunite.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.mall.tab3.playerunite.v1.PlayViewUniteReq".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.mall.tab3.playerunite.v1.PlayViewUniteReq".into()
    }
}
/// Generated client implementations.
pub mod player_client {
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
    pub struct PlayerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> PlayerClient<T>
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
        ) -> PlayerClient<InterceptedService<T, F>>
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
            PlayerClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn play_view_unite(
            &mut self,
            request: impl tonic::IntoRequest<super::PlayViewUniteReq>,
        ) -> std::result::Result<
            tonic::Response<super::PlayViewUniteReply>,
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
                "/bilibili.mall.tab3.playerunite.v1.Player/PlayViewUnite",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.mall.tab3.playerunite.v1.Player",
                        "PlayViewUnite",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod player_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with PlayerServer.
    #[async_trait]
    pub trait Player: std::marker::Send + std::marker::Sync + 'static {
        ///
        async fn play_view_unite(
            &self,
            request: tonic::Request<super::PlayViewUniteReq>,
        ) -> std::result::Result<
            tonic::Response<super::PlayViewUniteReply>,
            tonic::Status,
        >;
    }
    ///
    #[derive(Debug)]
    pub struct PlayerServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> PlayerServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PlayerServer<T>
    where
        T: Player,
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
                "/bilibili.mall.tab3.playerunite.v1.Player/PlayViewUnite" => {
                    #[allow(non_camel_case_types)]
                    struct PlayViewUniteSvc<T: Player>(pub Arc<T>);
                    impl<T: Player> tonic::server::UnaryService<super::PlayViewUniteReq>
                    for PlayViewUniteSvc<T> {
                        type Response = super::PlayViewUniteReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PlayViewUniteReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Player>::play_view_unite(&inner, request).await
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
                        let method = PlayViewUniteSvc(inner);
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
    impl<T> Clone for PlayerServer<T> {
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
    pub const SERVICE_NAME: &str = "bilibili.mall.tab3.playerunite.v1.Player";
    impl<T> tonic::server::NamedService for PlayerServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
