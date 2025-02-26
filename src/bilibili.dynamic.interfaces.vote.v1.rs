// This file is @generated by prost-build.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewDoVoteReq {
    ///
    #[prost(int64, tag = "1")]
    pub vote_id: i64,
    ///
    #[prost(int32, repeated, tag = "2")]
    pub votes: ::prost::alloc::vec::Vec<i32>,
    ///
    #[prost(int32, tag = "3")]
    pub status: i32,
    ///
    #[prost(int64, tag = "4")]
    pub dynamic_id: i64,
    ///
    #[prost(int64, tag = "5")]
    pub op_bit: i64,
}
impl ::prost::Name for NewDoVoteReq {
    const NAME: &'static str = "NewDoVoteReq";
    const PACKAGE: &'static str = "bilibili.dynamic.interfaces.vote.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.dynamic.interfaces.vote.v1.NewDoVoteReq".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.dynamic.interfaces.vote.v1.NewDoVoteReq".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewDoVoteRsp {
    ///
    #[prost(int64, tag = "1")]
    pub uid: i64,
    ///
    #[prost(int32, tag = "2")]
    pub r#type: i32,
    ///
    #[prost(message, optional, tag = "3")]
    pub vote_info: ::core::option::Option<VoteInfo>,
}
impl ::prost::Name for NewDoVoteRsp {
    const NAME: &'static str = "NewDoVoteRsp";
    const PACKAGE: &'static str = "bilibili.dynamic.interfaces.vote.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.dynamic.interfaces.vote.v1.NewDoVoteRsp".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.dynamic.interfaces.vote.v1.NewDoVoteRsp".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoteBizInfo {
    ///
    #[prost(int64, tag = "1")]
    pub biz_id: i64,
    ///
    #[prost(int32, tag = "2")]
    pub biz_type: i32,
    ///
    #[prost(string, tag = "3")]
    pub biz_ext: ::prost::alloc::string::String,
}
impl ::prost::Name for VoteBizInfo {
    const NAME: &'static str = "VoteBizInfo";
    const PACKAGE: &'static str = "bilibili.dynamic.interfaces.vote.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.dynamic.interfaces.vote.v1.VoteBizInfo".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.dynamic.interfaces.vote.v1.VoteBizInfo".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoteInfo {
    ///
    #[prost(int64, tag = "1")]
    pub vote_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "4")]
    pub join_num: i64,
    ///
    #[prost(int32, tag = "5")]
    pub r#type: i32,
    ///
    #[prost(int32, tag = "6")]
    pub choice_cnt: i32,
    ///
    #[prost(int64, tag = "7")]
    pub end_time: i64,
    ///
    #[prost(int32, tag = "8")]
    pub status: i32,
    ///
    #[prost(int64, tag = "9")]
    pub vote_publisher: i64,
    ///
    #[prost(int32, tag = "10")]
    pub default_share: i32,
    ///
    #[prost(int64, tag = "11")]
    pub ctime: i64,
    ///
    #[prost(int32, tag = "12")]
    pub biz_type: i32,
    ///
    #[prost(string, tag = "13")]
    pub img_url: ::prost::alloc::string::String,
    ///
    #[prost(int32, repeated, tag = "14")]
    pub my_votes: ::prost::alloc::vec::Vec<i32>,
    ///
    #[prost(message, repeated, tag = "15")]
    pub options: ::prost::alloc::vec::Vec<VoteOptionInfo>,
    ///
    #[prost(int32, tag = "16")]
    pub options_cnt: i32,
    ///
    #[prost(int64, tag = "17")]
    pub start_time: i64,
    ///
    #[prost(int64, tag = "18")]
    pub voter_uid: i64,
    ///
    #[prost(int32, tag = "19")]
    pub voter_level: i32,
    ///
    #[prost(int64, tag = "20")]
    pub duration: i64,
    ///
    #[prost(message, repeated, tag = "21")]
    pub vote_biz_info: ::prost::alloc::vec::Vec<VoteBizInfo>,
    ///
    #[prost(string, tag = "22")]
    pub face: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "23")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "24")]
    pub only_fans_level: i64,
}
impl ::prost::Name for VoteInfo {
    const NAME: &'static str = "VoteInfo";
    const PACKAGE: &'static str = "bilibili.dynamic.interfaces.vote.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.dynamic.interfaces.vote.v1.VoteInfo".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.dynamic.interfaces.vote.v1.VoteInfo".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoteOptionInfo {
    ///
    #[prost(int32, tag = "1")]
    pub opt_idx: i32,
    ///
    #[prost(string, tag = "2")]
    pub opt_desc: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub img_url: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "4")]
    pub cnt: i32,
    ///
    #[prost(string, tag = "5")]
    pub btn_str: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub title: ::prost::alloc::string::String,
}
impl ::prost::Name for VoteOptionInfo {
    const NAME: &'static str = "VoteOptionInfo";
    const PACKAGE: &'static str = "bilibili.dynamic.interfaces.vote.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.dynamic.interfaces.vote.v1.VoteOptionInfo".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.dynamic.interfaces.vote.v1.VoteOptionInfo".into()
    }
}
/// Generated client implementations.
pub mod vote_client {
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
    pub struct VoteClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> VoteClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
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
        ) -> VoteClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            VoteClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn new_do_vote(
            &mut self,
            request: impl tonic::IntoRequest<super::NewDoVoteReq>,
        ) -> std::result::Result<tonic::Response<super::NewDoVoteRsp>, tonic::Status> {
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
                "/bilibili.dynamic.interfaces.vote.v1.Vote/NewDoVote",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.dynamic.interfaces.vote.v1.Vote",
                        "NewDoVote",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod vote_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with VoteServer.
    #[async_trait]
    pub trait Vote: std::marker::Send + std::marker::Sync + 'static {
        ///
        async fn new_do_vote(
            &self,
            request: tonic::Request<super::NewDoVoteReq>,
        ) -> std::result::Result<tonic::Response<super::NewDoVoteRsp>, tonic::Status>;
    }
    ///
    #[derive(Debug)]
    pub struct VoteServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> VoteServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for VoteServer<T>
    where
        T: Vote,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
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
                "/bilibili.dynamic.interfaces.vote.v1.Vote/NewDoVote" => {
                    #[allow(non_camel_case_types)]
                    struct NewDoVoteSvc<T: Vote>(pub Arc<T>);
                    impl<T: Vote> tonic::server::UnaryService<super::NewDoVoteReq>
                    for NewDoVoteSvc<T> {
                        type Response = super::NewDoVoteRsp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewDoVoteReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Vote>::new_do_vote(&inner, request).await
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
                        let method = NewDoVoteSvc(inner);
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
                        let mut response = http::Response::new(empty_body());
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
    impl<T> Clone for VoteServer<T> {
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
    pub const SERVICE_NAME: &str = "bilibili.dynamic.interfaces.vote.v1.Vote";
    impl<T> tonic::server::NamedService for VoteServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
