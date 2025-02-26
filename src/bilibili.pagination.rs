// This file is @generated by prost-build.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedPagination {
    ///
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    ///
    #[prost(string, tag = "2")]
    pub offset: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "3")]
    pub is_refresh: bool,
}
impl ::prost::Name for FeedPagination {
    const NAME: &'static str = "FeedPagination";
    const PACKAGE: &'static str = "bilibili.pagination";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.pagination.FeedPagination".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.pagination.FeedPagination".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedPaginationReply {
    ///
    #[prost(string, tag = "1")]
    pub next_offset: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub prev_offset: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub last_read_offset: ::prost::alloc::string::String,
}
impl ::prost::Name for FeedPaginationReply {
    const NAME: &'static str = "FeedPaginationReply";
    const PACKAGE: &'static str = "bilibili.pagination";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.pagination.FeedPaginationReply".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.pagination.FeedPaginationReply".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pagination {
    ///
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    ///
    #[prost(string, tag = "2")]
    pub next: ::prost::alloc::string::String,
}
impl ::prost::Name for Pagination {
    const NAME: &'static str = "Pagination";
    const PACKAGE: &'static str = "bilibili.pagination";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.pagination.Pagination".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.pagination.Pagination".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaginationReply {
    ///
    #[prost(string, tag = "1")]
    pub next: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub prev: ::prost::alloc::string::String,
}
impl ::prost::Name for PaginationReply {
    const NAME: &'static str = "PaginationReply";
    const PACKAGE: &'static str = "bilibili.pagination";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.pagination.PaginationReply".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.pagination.PaginationReply".into()
    }
}
