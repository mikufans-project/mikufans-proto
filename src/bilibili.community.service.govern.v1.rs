// This file is @generated by prost-build.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceInfo {
    ///
    #[prost(string, tag = "1")]
    pub buvid: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub build: i64,
    ///
    #[prost(string, tag = "3")]
    pub raw_platform: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub raw_mobi_app: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub device: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub channel: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub buvid3: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub user_agent: ::prost::alloc::string::String,
}
impl ::prost::Name for DeviceInfo {
    const NAME: &'static str = "DeviceInfo";
    const PACKAGE: &'static str = "bilibili.community.service.govern.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.community.service.govern.v1.DeviceInfo".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.community.service.govern.v1.DeviceInfo".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QoeReportReq {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub scene: i64,
    ///
    #[prost(int32, tag = "3")]
    pub r#type: i32,
    ///
    #[prost(bool, tag = "4")]
    pub cancel: bool,
    ///
    #[prost(string, tag = "5")]
    pub business_type: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "6")]
    pub oid: i64,
    ///
    #[prost(message, optional, tag = "7")]
    pub score_result: ::core::option::Option<QoeScoreResult>,
    ///
    #[prost(string, tag = "8")]
    pub business_data: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "9")]
    pub result: ::core::option::Option<QoeResult>,
    ///
    #[prost(message, optional, tag = "10")]
    pub device_info: ::core::option::Option<DeviceInfo>,
}
impl ::prost::Name for QoeReportReq {
    const NAME: &'static str = "QoeReportReq";
    const PACKAGE: &'static str = "bilibili.community.service.govern.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.community.service.govern.v1.QoeReportReq".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.community.service.govern.v1.QoeReportReq".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QoeResult {
    ///
    #[prost(float, tag = "1")]
    pub option_score: f32,
    ///
    #[prost(string, tag = "2")]
    pub option_title: ::prost::alloc::string::String,
    ///
    #[prost(string, repeated, tag = "3")]
    pub option_descs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(string, tag = "4")]
    pub option_custom_feedback: ::prost::alloc::string::String,
}
impl ::prost::Name for QoeResult {
    const NAME: &'static str = "QoeResult";
    const PACKAGE: &'static str = "bilibili.community.service.govern.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.community.service.govern.v1.QoeResult".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.community.service.govern.v1.QoeResult".into()
    }
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct QoeScoreResult {
    ///
    #[prost(float, tag = "1")]
    pub score: f32,
}
impl ::prost::Name for QoeScoreResult {
    const NAME: &'static str = "QoeScoreResult";
    const PACKAGE: &'static str = "bilibili.community.service.govern.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.community.service.govern.v1.QoeScoreResult".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.community.service.govern.v1.QoeScoreResult".into()
    }
}
/// Generated client implementations.
pub mod qoe_client {
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
    pub struct QoeClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> QoeClient<T>
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
        ) -> QoeClient<InterceptedService<T, F>>
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
            QoeClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn qoe_report(
            &mut self,
            request: impl tonic::IntoRequest<super::QoeReportReq>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/bilibili.community.service.govern.v1.Qoe/QoeReport",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.community.service.govern.v1.Qoe",
                        "QoeReport",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod qoe_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with QoeServer.
    #[async_trait]
    pub trait Qoe: std::marker::Send + std::marker::Sync + 'static {
        ///
        async fn qoe_report(
            &self,
            request: tonic::Request<super::QoeReportReq>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
    }
    ///
    #[derive(Debug)]
    pub struct QoeServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> QoeServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for QoeServer<T>
    where
        T: Qoe,
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
                "/bilibili.community.service.govern.v1.Qoe/QoeReport" => {
                    #[allow(non_camel_case_types)]
                    struct QoeReportSvc<T: Qoe>(pub Arc<T>);
                    impl<T: Qoe> tonic::server::UnaryService<super::QoeReportReq>
                    for QoeReportSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QoeReportReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Qoe>::qoe_report(&inner, request).await
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
                        let method = QoeReportSvc(inner);
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
    impl<T> Clone for QoeServer<T> {
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
    pub const SERVICE_NAME: &str = "bilibili.community.service.govern.v1.Qoe";
    impl<T> tonic::server::NamedService for QoeServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
