// This file is @generated by prost-build.
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct BasicRenderSpec {
    ///
    #[prost(double, tag = "1")]
    pub opacity: f64,
}
impl ::prost::Name for BasicRenderSpec {
    const NAME: &'static str = "BasicRenderSpec";
    const PACKAGE: &'static str = "bilibili.dagw.component.avatar.common";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.dagw.component.avatar.common.BasicRenderSpec".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.dagw.component.avatar.common.BasicRenderSpec".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColorConfig {
    ///
    #[prost(bool, tag = "1")]
    pub is_dark_mode_aware: bool,
    ///
    #[prost(message, optional, tag = "2")]
    pub day: ::core::option::Option<ColorSpec>,
    ///
    #[prost(message, optional, tag = "3")]
    pub night: ::core::option::Option<ColorSpec>,
}
impl ::prost::Name for ColorConfig {
    const NAME: &'static str = "ColorConfig";
    const PACKAGE: &'static str = "bilibili.dagw.component.avatar.common";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.dagw.component.avatar.common.ColorConfig".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.dagw.component.avatar.common.ColorConfig".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColorSpec {
    ///
    #[prost(string, tag = "1")]
    pub argb: ::prost::alloc::string::String,
}
impl ::prost::Name for ColorSpec {
    const NAME: &'static str = "ColorSpec";
    const PACKAGE: &'static str = "bilibili.dagw.component.avatar.common";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.dagw.component.avatar.common.ColorSpec".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.dagw.component.avatar.common.ColorSpec".into()
    }
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct LayerGeneralSpec {
    ///
    #[prost(message, optional, tag = "1")]
    pub pos_spec: ::core::option::Option<PositionSpec>,
    ///
    #[prost(message, optional, tag = "2")]
    pub size_spec: ::core::option::Option<SizeSpec>,
    ///
    #[prost(message, optional, tag = "3")]
    pub render_spec: ::core::option::Option<BasicRenderSpec>,
}
impl ::prost::Name for LayerGeneralSpec {
    const NAME: &'static str = "LayerGeneralSpec";
    const PACKAGE: &'static str = "bilibili.dagw.component.avatar.common";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.dagw.component.avatar.common.LayerGeneralSpec".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.dagw.component.avatar.common.LayerGeneralSpec".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaskProperty {
    ///
    #[prost(message, optional, tag = "1")]
    pub general_spec: ::core::option::Option<LayerGeneralSpec>,
    ///
    #[prost(message, optional, tag = "2")]
    pub mask_src: ::core::option::Option<ResourceSource>,
}
impl ::prost::Name for MaskProperty {
    const NAME: &'static str = "MaskProperty";
    const PACKAGE: &'static str = "bilibili.dagw.component.avatar.common";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.dagw.component.avatar.common.MaskProperty".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.dagw.component.avatar.common.MaskProperty".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativeDrawRes {
    ///
    #[prost(enumeration = "native_draw_res::NativeDraw", tag = "1")]
    pub draw_type: i32,
    ///
    #[prost(enumeration = "native_draw_res::FillMode", tag = "2")]
    pub fill_mode: i32,
    ///
    #[prost(message, optional, tag = "3")]
    pub color_config: ::core::option::Option<ColorConfig>,
    ///
    #[prost(double, tag = "4")]
    pub edge_weight: f64,
}
/// Nested message and enum types in `NativeDrawRes`.
pub mod native_draw_res {
    ///
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum FillMode {
        ///
        Invalid = 0,
        ///
        Internal = 1,
        ///
        Edge = 2,
    }
    impl FillMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Invalid => "FILL_MODE_INVALID",
                Self::Internal => "FILL_MODE_INTERNAL",
                Self::Edge => "FILL_MODE_EDGE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FILL_MODE_INVALID" => Some(Self::Invalid),
                "FILL_MODE_INTERNAL" => Some(Self::Internal),
                "FILL_MODE_EDGE" => Some(Self::Edge),
                _ => None,
            }
        }
    }
    ///
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum NativeDraw {
        ///
        DrawInvalid = 0,
        ///
        DrawCircle = 1,
        ///
        DrawRectangle = 2,
    }
    impl NativeDraw {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::DrawInvalid => "DRAW_INVALID",
                Self::DrawCircle => "DRAW_CIRCLE",
                Self::DrawRectangle => "DRAW_RECTANGLE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DRAW_INVALID" => Some(Self::DrawInvalid),
                "DRAW_CIRCLE" => Some(Self::DrawCircle),
                "DRAW_RECTANGLE" => Some(Self::DrawRectangle),
                _ => None,
            }
        }
    }
}
impl ::prost::Name for NativeDrawRes {
    const NAME: &'static str = "NativeDrawRes";
    const PACKAGE: &'static str = "bilibili.dagw.component.avatar.common";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.dagw.component.avatar.common.NativeDrawRes".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.dagw.component.avatar.common.NativeDrawRes".into()
    }
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PositionSpec {
    ///
    #[prost(enumeration = "position_spec::CoordinatePos", tag = "1")]
    pub coordinate_pos: i32,
    ///
    #[prost(double, tag = "2")]
    pub axis_x: f64,
    ///
    #[prost(double, tag = "3")]
    pub axis_y: f64,
}
/// Nested message and enum types in `PositionSpec`.
pub mod position_spec {
    ///
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum CoordinatePos {
        ///
        InvalidCoordinate = 0,
        ///
        DefaultCoordinate = 1,
        ///
        CentralCoordinate = 2,
    }
    impl CoordinatePos {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::InvalidCoordinate => "INVALID_COORDINATE",
                Self::DefaultCoordinate => "DEFAULT_COORDINATE",
                Self::CentralCoordinate => "CENTRAL_COORDINATE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INVALID_COORDINATE" => Some(Self::InvalidCoordinate),
                "DEFAULT_COORDINATE" => Some(Self::DefaultCoordinate),
                "CENTRAL_COORDINATE" => Some(Self::CentralCoordinate),
                _ => None,
            }
        }
    }
}
impl ::prost::Name for PositionSpec {
    const NAME: &'static str = "PositionSpec";
    const PACKAGE: &'static str = "bilibili.dagw.component.avatar.common";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.dagw.component.avatar.common.PositionSpec".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.dagw.component.avatar.common.PositionSpec".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteRes {
    ///
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub bfs_style: ::prost::alloc::string::String,
}
impl ::prost::Name for RemoteRes {
    const NAME: &'static str = "RemoteRes";
    const PACKAGE: &'static str = "bilibili.dagw.component.avatar.common";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.dagw.component.avatar.common.RemoteRes".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.dagw.component.avatar.common.RemoteRes".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceSource {
    ///
    #[prost(enumeration = "resource_source::SourceType", tag = "1")]
    pub src_type: i32,
    ///
    #[prost(enumeration = "resource_source::LocalRes", tag = "2")]
    pub placeholder: i32,
    ///
    #[prost(oneof = "resource_source::Res", tags = "3, 4, 5")]
    pub res: ::core::option::Option<resource_source::Res>,
}
/// Nested message and enum types in `ResourceSource`.
pub mod resource_source {
    ///
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum LocalRes {
        ///
        Invalid = 0,
        ///
        IconVip = 1,
        ///
        IconSmallVip = 2,
        ///
        IconPersonalVerify = 3,
        ///
        IconEnterpriseVerify = 4,
        ///
        IconNftMainland = 5,
        ///
        DefaultAvatar = 6,
        ///
        FollowIcon = 7,
        ///
        FollowAction = 8,
    }
    impl LocalRes {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Invalid => "LOCAL_RES_INVALID",
                Self::IconVip => "LOCAL_RES_ICON_VIP",
                Self::IconSmallVip => "LOCAL_RES_ICON_SMALL_VIP",
                Self::IconPersonalVerify => "LOCAL_RES_ICON_PERSONAL_VERIFY",
                Self::IconEnterpriseVerify => "LOCAL_RES_ICON_ENTERPRISE_VERIFY",
                Self::IconNftMainland => "LOCAL_RES_ICON_NFT_MAINLAND",
                Self::DefaultAvatar => "LOCAL_RES_DEFAULT_AVATAR",
                Self::FollowIcon => "LOCAL_RES_FOLLOW_ICON",
                Self::FollowAction => "LOCAL_RES_FOLLOW_ACTION",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LOCAL_RES_INVALID" => Some(Self::Invalid),
                "LOCAL_RES_ICON_VIP" => Some(Self::IconVip),
                "LOCAL_RES_ICON_SMALL_VIP" => Some(Self::IconSmallVip),
                "LOCAL_RES_ICON_PERSONAL_VERIFY" => Some(Self::IconPersonalVerify),
                "LOCAL_RES_ICON_ENTERPRISE_VERIFY" => Some(Self::IconEnterpriseVerify),
                "LOCAL_RES_ICON_NFT_MAINLAND" => Some(Self::IconNftMainland),
                "LOCAL_RES_DEFAULT_AVATAR" => Some(Self::DefaultAvatar),
                "LOCAL_RES_FOLLOW_ICON" => Some(Self::FollowIcon),
                "LOCAL_RES_FOLLOW_ACTION" => Some(Self::FollowAction),
                _ => None,
            }
        }
    }
    ///
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum SourceType {
        ///
        SrcTypeInvalid = 0,
        ///
        SrcTypeUrl = 1,
        ///
        SrcTypeLocal = 2,
        ///
        SrcTypeDraw = 3,
    }
    impl SourceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::SrcTypeInvalid => "SRC_TYPE_INVALID",
                Self::SrcTypeUrl => "SRC_TYPE_URL",
                Self::SrcTypeLocal => "SRC_TYPE_LOCAL",
                Self::SrcTypeDraw => "SRC_TYPE_DRAW",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SRC_TYPE_INVALID" => Some(Self::SrcTypeInvalid),
                "SRC_TYPE_URL" => Some(Self::SrcTypeUrl),
                "SRC_TYPE_LOCAL" => Some(Self::SrcTypeLocal),
                "SRC_TYPE_DRAW" => Some(Self::SrcTypeDraw),
                _ => None,
            }
        }
    }
    ///
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Res {
        ///
        #[prost(message, tag = "3")]
        Remote(super::RemoteRes),
        ///
        #[prost(int32, tag = "4")]
        LocalValue(i32),
        ///
        #[prost(message, tag = "5")]
        Draw(super::NativeDrawRes),
    }
}
impl ::prost::Name for ResourceSource {
    const NAME: &'static str = "ResourceSource";
    const PACKAGE: &'static str = "bilibili.dagw.component.avatar.common";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.dagw.component.avatar.common.ResourceSource".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.dagw.component.avatar.common.ResourceSource".into()
    }
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct SizeSpec {
    ///
    #[prost(double, tag = "1")]
    pub width: f64,
    ///
    #[prost(double, tag = "2")]
    pub height: f64,
}
impl ::prost::Name for SizeSpec {
    const NAME: &'static str = "SizeSpec";
    const PACKAGE: &'static str = "bilibili.dagw.component.avatar.common";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.dagw.component.avatar.common.SizeSpec".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.dagw.component.avatar.common.SizeSpec".into()
    }
}
