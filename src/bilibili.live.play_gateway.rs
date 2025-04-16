// This file is @generated by prost-build.
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Behavior {
    ///
    #[prost(int64, tag = "1")]
    pub qn: i64,
    ///
    #[prost(enumeration = "PlayStyle", tag = "2")]
    pub play_style: i32,
}
impl ::prost::Name for Behavior {
    const NAME: &'static str = "Behavior";
    const PACKAGE: &'static str = "bilibili.live.play_gateway";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.live.play_gateway.Behavior".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.live.play_gateway.Behavior".into()
    }
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct GetMasterPlayQualityReq {
    ///
    #[prost(int64, tag = "1")]
    pub room_id: i64,
}
impl ::prost::Name for GetMasterPlayQualityReq {
    const NAME: &'static str = "GetMasterPlayQualityReq";
    const PACKAGE: &'static str = "bilibili.live.play_gateway";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.live.play_gateway.GetMasterPlayQualityReq".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.live.play_gateway.GetMasterPlayQualityReq".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMasterPlayQualityResp {
    ///
    #[prost(message, repeated, tag = "1")]
    pub behaviors: ::prost::alloc::vec::Vec<Behavior>,
    ///
    #[prost(message, repeated, tag = "2")]
    pub qoes: ::prost::alloc::vec::Vec<Qoe>,
}
impl ::prost::Name for GetMasterPlayQualityResp {
    const NAME: &'static str = "GetMasterPlayQualityResp";
    const PACKAGE: &'static str = "bilibili.live.play_gateway";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.live.play_gateway.GetMasterPlayQualityResp".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.live.play_gateway.GetMasterPlayQualityResp".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Qoe {
    ///
    #[prost(string, tag = "1")]
    pub qoe: ::prost::alloc::string::String,
}
impl ::prost::Name for Qoe {
    const NAME: &'static str = "Qoe";
    const PACKAGE: &'static str = "bilibili.live.play_gateway";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.live.play_gateway.Qoe".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.live.play_gateway.Qoe".into()
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PlayStyle {
    ///
    Horriz = 0,
    ///
    Vert = 1,
    ///
    Tinywindow = 2,
}
impl PlayStyle {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Horriz => "PLAY_STYLE_HORRIZ",
            Self::Vert => "PLAY_STYLE_VERT",
            Self::Tinywindow => "PLAY_STYLE_TINYWINDOW",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PLAY_STYLE_HORRIZ" => Some(Self::Horriz),
            "PLAY_STYLE_VERT" => Some(Self::Vert),
            "PLAY_STYLE_TINYWINDOW" => Some(Self::Tinywindow),
            _ => None,
        }
    }
}
