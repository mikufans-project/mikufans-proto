// This file is @generated by prost-build.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudPlayConfig {
    ///
    #[prost(message, optional, tag = "1")]
    pub enable_panorama: ::core::option::Option<super::super::BoolValue>,
    ///
    #[prost(message, optional, tag = "2")]
    pub enable_dolby: ::core::option::Option<super::super::BoolValue>,
    ///
    #[prost(message, optional, tag = "3")]
    pub enable_shake: ::core::option::Option<super::super::BoolValue>,
    ///
    #[prost(message, optional, tag = "4")]
    pub enable_background: ::core::option::Option<super::super::BoolValue>,
    ///
    #[prost(message, optional, tag = "5")]
    pub enable_loss_less: ::core::option::Option<super::super::BoolValue>,
    ///
    #[prost(message, optional, tag = "6")]
    pub priority_use_dolby_h_d_r: ::core::option::Option<super::super::BoolValue>,
}
impl ::prost::Name for CloudPlayConfig {
    const NAME: &'static str = "CloudPlayConfig";
    const PACKAGE: &'static str = "bilibili.app.distribution.setting.play";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.distribution.setting.play.CloudPlayConfig".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.distribution.setting.play.CloudPlayConfig".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MidPlayConfig {
    ///
    #[prost(message, optional, tag = "1")]
    pub dynamic_to_playlist: ::core::option::Option<super::super::BoolValue>,
}
impl ::prost::Name for MidPlayConfig {
    const NAME: &'static str = "MidPlayConfig";
    const PACKAGE: &'static str = "bilibili.app.distribution.setting.play";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.distribution.setting.play.MidPlayConfig".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.distribution.setting.play.MidPlayConfig".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayConfig {
    ///
    #[prost(message, optional, tag = "1")]
    pub should_auto_play: ::core::option::Option<super::super::BoolValue>,
    ///
    #[prost(message, optional, tag = "2")]
    pub should_auto_full_screen: ::core::option::Option<super::super::BoolValue>,
    ///
    #[prost(message, optional, tag = "3")]
    pub enable_playurl_h_t_t_p_s: ::core::option::Option<super::super::BoolValue>,
    ///
    #[prost(message, optional, tag = "4")]
    pub enable_danmaku_interaction: ::core::option::Option<super::super::BoolValue>,
    ///
    #[prost(message, optional, tag = "5")]
    pub small_screen_status: ::core::option::Option<super::super::Int64Value>,
    ///
    #[prost(message, optional, tag = "6")]
    pub player_codec_mode_key: ::core::option::Option<super::super::Int64Value>,
    ///
    #[prost(message, optional, tag = "7")]
    pub enable_gravity_rotate_screen: ::core::option::Option<super::super::BoolValue>,
    ///
    #[prost(message, optional, tag = "8")]
    pub enable_danmaku_monospaced: ::core::option::Option<super::super::BoolValue>,
    ///
    #[prost(message, optional, tag = "9")]
    pub enable_edit_subtitle: ::core::option::Option<super::super::BoolValue>,
    ///
    #[prost(message, optional, tag = "10")]
    pub enable_subtitle: ::core::option::Option<super::super::BoolValue>,
    ///
    #[prost(message, optional, tag = "11")]
    pub color_filter: ::core::option::Option<super::super::Int64Value>,
    ///
    #[prost(message, optional, tag = "12")]
    pub should_auto_story: ::core::option::Option<super::super::BoolValue>,
    ///
    #[prost(message, optional, tag = "13")]
    pub landscape_auto_story: ::core::option::Option<super::super::BoolValue>,
    ///
    #[prost(message, optional, tag = "14")]
    pub volume_balance: ::core::option::Option<super::super::BoolValue>,
    ///
    #[prost(message, optional, tag = "15")]
    pub volume_balance_mode: ::core::option::Option<super::super::Int64Value>,
}
impl ::prost::Name for PlayConfig {
    const NAME: &'static str = "PlayConfig";
    const PACKAGE: &'static str = "bilibili.app.distribution.setting.play";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.distribution.setting.play.PlayConfig".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.distribution.setting.play.PlayConfig".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpecificPlayConfig {
    ///
    #[prost(message, optional, tag = "1")]
    pub enable_segmented_section: ::core::option::Option<super::super::BoolValue>,
}
impl ::prost::Name for SpecificPlayConfig {
    const NAME: &'static str = "SpecificPlayConfig";
    const PACKAGE: &'static str = "bilibili.app.distribution.setting.play";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.distribution.setting.play.SpecificPlayConfig".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.distribution.setting.play.SpecificPlayConfig".into()
    }
}
