// This file is @generated by prost-build.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessagePullsReply {
    ///
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<::prost_types::Any>,
    ///
    #[prost(int32, tag = "2")]
    pub pn: i32,
    ///
    #[prost(int32, tag = "3")]
    pub ps: i32,
    ///
    #[prost(int64, tag = "4")]
    pub count: i64,
    ///
    #[prost(bool, tag = "5")]
    pub has_next: bool,
}
impl ::prost::Name for MessagePullsReply {
    const NAME: &'static str = "MessagePullsReply";
    const PACKAGE: &'static str = "bilibili.vega.deneb.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.vega.deneb.v1.MessagePullsReply".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.vega.deneb.v1.MessagePullsReply".into()
    }
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct MessagePullsReq {
    ///
    #[prost(int64, tag = "1")]
    pub start_seq_id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub end_seq_id: i64,
    ///
    #[prost(int32, tag = "3")]
    pub pn: i32,
    ///
    #[prost(int32, tag = "4")]
    pub ps: i32,
}
impl ::prost::Name for MessagePullsReq {
    const NAME: &'static str = "MessagePullsReq";
    const PACKAGE: &'static str = "bilibili.vega.deneb.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.vega.deneb.v1.MessagePullsReq".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.vega.deneb.v1.MessagePullsReq".into()
    }
}
/// Generated client implementations.
pub mod vega_deneb_rpc_client {
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
    pub struct VegaDenebRpcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> VegaDenebRpcClient<T>
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
        ) -> VegaDenebRpcClient<InterceptedService<T, F>>
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
            VegaDenebRpcClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn message_pulls(
            &mut self,
            request: impl tonic::IntoRequest<super::MessagePullsReq>,
        ) -> std::result::Result<
            tonic::Response<super::MessagePullsReply>,
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
                "/bilibili.vega.deneb.v1.VegaDenebRPC/MessagePulls",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.vega.deneb.v1.VegaDenebRPC",
                        "MessagePulls",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod vega_deneb_rpc_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with VegaDenebRpcServer.
    #[async_trait]
    pub trait VegaDenebRpc: std::marker::Send + std::marker::Sync + 'static {
        ///
        async fn message_pulls(
            &self,
            request: tonic::Request<super::MessagePullsReq>,
        ) -> std::result::Result<
            tonic::Response<super::MessagePullsReply>,
            tonic::Status,
        >;
    }
    ///
    #[derive(Debug)]
    pub struct VegaDenebRpcServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> VegaDenebRpcServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for VegaDenebRpcServer<T>
    where
        T: VegaDenebRpc,
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
                "/bilibili.vega.deneb.v1.VegaDenebRPC/MessagePulls" => {
                    #[allow(non_camel_case_types)]
                    struct MessagePullsSvc<T: VegaDenebRpc>(pub Arc<T>);
                    impl<
                        T: VegaDenebRpc,
                    > tonic::server::UnaryService<super::MessagePullsReq>
                    for MessagePullsSvc<T> {
                        type Response = super::MessagePullsReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MessagePullsReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as VegaDenebRpc>::message_pulls(&inner, request).await
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
                        let method = MessagePullsSvc(inner);
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
    impl<T> Clone for VegaDenebRpcServer<T> {
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
    pub const SERVICE_NAME: &str = "bilibili.vega.deneb.v1.VegaDenebRPC";
    impl<T> tonic::server::NamedService for VegaDenebRpcServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
