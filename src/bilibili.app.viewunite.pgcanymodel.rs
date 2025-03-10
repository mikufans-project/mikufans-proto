// This file is @generated by prost-build.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Earphone {
    ///
    #[prost(string, tag = "1")]
    pub product_model: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub like_toast_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub switch_toast_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub like_toast_voice: ::prost::alloc::string::String,
}
impl ::prost::Name for Earphone {
    const NAME: &'static str = "Earphone";
    const PACKAGE: &'static str = "bilibili.app.viewunite.pgcanymodel";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.viewunite.pgcanymodel.Earphone".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.viewunite.pgcanymodel.Earphone".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EarphoneConf {
    ///
    #[prost(message, repeated, tag = "1")]
    pub sp_phones: ::prost::alloc::vec::Vec<Earphone>,
}
impl ::prost::Name for EarphoneConf {
    const NAME: &'static str = "EarphoneConf";
    const PACKAGE: &'static str = "bilibili.app.viewunite.pgcanymodel";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.viewunite.pgcanymodel.EarphoneConf".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.viewunite.pgcanymodel.EarphoneConf".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiViewInfo {
    ///
    #[prost(bool, tag = "1")]
    pub is_multi_view_season: bool,
    ///
    #[prost(string, tag = "2")]
    pub changing_dance: ::prost::alloc::string::String,
}
impl ::prost::Name for MultiViewInfo {
    const NAME: &'static str = "MultiViewInfo";
    const PACKAGE: &'static str = "bilibili.app.viewunite.pgcanymodel";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.viewunite.pgcanymodel.MultiViewInfo".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.viewunite.pgcanymodel.MultiViewInfo".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OgvData {
    ///
    #[prost(int32, tag = "1")]
    pub media_id: i32,
    ///
    #[prost(int64, tag = "2")]
    pub season_id: i64,
    ///
    #[prost(int32, tag = "3")]
    pub season_type: i32,
    ///
    #[prost(int32, tag = "4")]
    pub show_season_type: i32,
    ///
    #[prost(message, optional, tag = "5")]
    pub rights: ::core::option::Option<Rights>,
    ///
    #[prost(message, optional, tag = "6")]
    pub user_status: ::core::option::Option<UserStatus>,
    ///
    #[prost(int64, tag = "7")]
    pub aid: i64,
    ///
    #[prost(message, optional, tag = "8")]
    pub stat: ::core::option::Option<Stat>,
    ///
    #[prost(int32, tag = "9")]
    pub mode: i32,
    ///
    #[prost(message, optional, tag = "10")]
    pub publish: ::core::option::Option<Publish>,
    ///
    #[prost(message, optional, tag = "11")]
    pub play_strategy: ::core::option::Option<PlayStrategy>,
    ///
    #[prost(message, optional, tag = "12")]
    pub multi_view_info: ::core::option::Option<MultiViewInfo>,
    ///
    #[prost(message, optional, tag = "13")]
    pub ogv_switch: ::core::option::Option<OgvSwitch>,
    ///
    #[prost(int32, tag = "14")]
    pub total_ep: i32,
    ///
    #[prost(message, optional, tag = "15")]
    pub new_ep: ::core::option::Option<super::common::NewEp>,
    ///
    #[prost(message, optional, tag = "16")]
    pub reserve: ::core::option::Option<Reserve>,
    ///
    #[prost(int32, tag = "17")]
    pub status: i32,
    ///
    #[prost(message, repeated, tag = "18")]
    pub activity_float_layer: ::prost::alloc::vec::Vec<PlayFloatLayerActivity>,
    ///
    #[prost(message, optional, tag = "19")]
    pub earphone_conf: ::core::option::Option<EarphoneConf>,
    ///
    #[prost(string, tag = "20")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "21")]
    pub square_cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "22")]
    pub share_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "23")]
    pub short_link: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "24")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "25")]
    pub horizontal_cover169: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "26")]
    pub horizontal_cover1610: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "27")]
    pub has_can_play_ep: i32,
    ///
    #[prost(message, optional, tag = "28")]
    pub skin: ::core::option::Option<Skin>,
}
impl ::prost::Name for OgvData {
    const NAME: &'static str = "OgvData";
    const PACKAGE: &'static str = "bilibili.app.viewunite.pgcanymodel";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.viewunite.pgcanymodel.OgvData".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.viewunite.pgcanymodel.OgvData".into()
    }
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct OgvSwitch {
    ///
    #[prost(int32, tag = "1")]
    pub reduce_short_title_spacing: i32,
    ///
    #[prost(int32, tag = "2")]
    pub merge_position_section_for_cinema: i32,
    ///
    #[prost(int32, tag = "3")]
    pub merge_preview_section: i32,
    ///
    #[prost(int32, tag = "4")]
    pub enable_show_vt_info: i32,
    ///
    #[prost(int32, tag = "5")]
    pub hide_ep_vv_vt_dm: i32,
    ///
    #[prost(int32, tag = "6")]
    pub follow_guide: i32,
}
impl ::prost::Name for OgvSwitch {
    const NAME: &'static str = "OgvSwitch";
    const PACKAGE: &'static str = "bilibili.app.viewunite.pgcanymodel";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.viewunite.pgcanymodel.OgvSwitch".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.viewunite.pgcanymodel.OgvSwitch".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayFloatLayerActivity {
    ///
    #[prost(int32, tag = "1")]
    pub id: i32,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "3")]
    pub r#type: i32,
    ///
    #[prost(int32, tag = "4")]
    pub ad_badge_type: i32,
    ///
    #[prost(string, tag = "5")]
    pub link: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub pic_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub pic_anima_url: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "8")]
    pub badge: ::core::option::Option<super::common::BadgeInfo>,
    ///
    #[prost(int64, tag = "9")]
    pub show_rate_time: i64,
}
impl ::prost::Name for PlayFloatLayerActivity {
    const NAME: &'static str = "PlayFloatLayerActivity";
    const PACKAGE: &'static str = "bilibili.app.viewunite.pgcanymodel";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.viewunite.pgcanymodel.PlayFloatLayerActivity".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.viewunite.pgcanymodel.PlayFloatLayerActivity".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayStrategy {
    ///
    #[prost(string, repeated, tag = "1")]
    pub strategies: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(int32, tag = "2")]
    pub recommend_show_strategy: i32,
    ///
    #[prost(string, tag = "3")]
    pub auto_play_toast: ::prost::alloc::string::String,
}
impl ::prost::Name for PlayStrategy {
    const NAME: &'static str = "PlayStrategy";
    const PACKAGE: &'static str = "bilibili.app.viewunite.pgcanymodel";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.viewunite.pgcanymodel.PlayStrategy".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.viewunite.pgcanymodel.PlayStrategy".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Publish {
    ///
    #[prost(string, tag = "1")]
    pub pub_time: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub pub_time_show: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "3")]
    pub is_started: i32,
    ///
    #[prost(int32, tag = "4")]
    pub is_finish: i32,
    ///
    #[prost(int32, tag = "5")]
    pub weekday: i32,
    ///
    #[prost(string, tag = "6")]
    pub release_date_show: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub time_length_show: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "8")]
    pub unknow_pub_date: i32,
    ///
    #[prost(string, tag = "9")]
    pub update_info_desc: ::prost::alloc::string::String,
}
impl ::prost::Name for Publish {
    const NAME: &'static str = "Publish";
    const PACKAGE: &'static str = "bilibili.app.viewunite.pgcanymodel";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.viewunite.pgcanymodel.Publish".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.viewunite.pgcanymodel.Publish".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reserve {
    ///
    #[prost(message, repeated, tag = "1")]
    pub episodes: ::prost::alloc::vec::Vec<super::common::ViewEpisode>,
    ///
    #[prost(string, tag = "2")]
    pub tip: ::prost::alloc::string::String,
}
impl ::prost::Name for Reserve {
    const NAME: &'static str = "Reserve";
    const PACKAGE: &'static str = "bilibili.app.viewunite.pgcanymodel";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.viewunite.pgcanymodel.Reserve".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.viewunite.pgcanymodel.Reserve".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rights {
    ///
    #[prost(int32, tag = "1")]
    pub allow_download: i32,
    ///
    #[prost(int32, tag = "2")]
    pub allow_review: i32,
    ///
    #[prost(int32, tag = "3")]
    pub can_watch: i32,
    ///
    #[prost(int32, tag = "4")]
    pub is_cover_show: i32,
    ///
    #[prost(string, tag = "5")]
    pub copyright: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub copyright_name: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "7")]
    pub allow_bp: i32,
    ///
    #[prost(int32, tag = "8")]
    pub area_limit: i32,
    ///
    #[prost(int32, tag = "9")]
    pub is_preview: i32,
    ///
    #[prost(int32, tag = "10")]
    pub ban_area_show: i32,
    ///
    #[prost(int32, tag = "11")]
    pub watch_platform: i32,
    ///
    #[prost(int32, tag = "12")]
    pub allow_bp_rank: i32,
    ///
    #[prost(string, tag = "13")]
    pub resource: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "14")]
    pub forbid_pre: i32,
    ///
    #[prost(int32, tag = "15")]
    pub only_vip_download: i32,
    ///
    #[prost(int32, tag = "16")]
    pub new_allow_download: i32,
}
impl ::prost::Name for Rights {
    const NAME: &'static str = "Rights";
    const PACKAGE: &'static str = "bilibili.app.viewunite.pgcanymodel";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.viewunite.pgcanymodel.Rights".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.viewunite.pgcanymodel.Rights".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Skin {
    ///
    #[prost(string, tag = "1")]
    pub tab_text_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub tab_text_night_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub bg_img: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub bg_img_night: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub dm_input_frame_bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub dm_input_frame_bg_night_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub dm_input_frame_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub dm_input_frame_night_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub dm_btn_bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub dm_btn_bg_night_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "11")]
    pub dm_btn_icon_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "12")]
    pub dm_btn_icon_night_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "13")]
    pub dm_input_text_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "14")]
    pub dm_input_text_night_color: ::prost::alloc::string::String,
}
impl ::prost::Name for Skin {
    const NAME: &'static str = "Skin";
    const PACKAGE: &'static str = "bilibili.app.viewunite.pgcanymodel";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.viewunite.pgcanymodel.Skin".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.viewunite.pgcanymodel.Skin".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Stat {
    ///
    #[prost(string, tag = "1")]
    pub followers: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "2")]
    pub play_data: ::core::option::Option<super::common::StatInfo>,
}
impl ::prost::Name for Stat {
    const NAME: &'static str = "Stat";
    const PACKAGE: &'static str = "bilibili.app.viewunite.pgcanymodel";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.viewunite.pgcanymodel.Stat".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.viewunite.pgcanymodel.Stat".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserStatus {
    ///
    #[prost(int32, tag = "1")]
    pub show: i32,
    ///
    #[prost(int32, tag = "2")]
    pub follow: i32,
    ///
    #[prost(int32, tag = "3")]
    pub follow_status: i32,
    ///
    #[prost(int32, tag = "4")]
    pub pay: i32,
    ///
    #[prost(int32, tag = "5")]
    pub sponsor: i32,
    ///
    #[prost(int32, tag = "6")]
    pub vip: i32,
    ///
    #[prost(int32, tag = "7")]
    pub vip_frozen: i32,
    ///
    #[prost(message, optional, tag = "8")]
    pub watch_progress: ::core::option::Option<WatchProgress>,
}
impl ::prost::Name for UserStatus {
    const NAME: &'static str = "UserStatus";
    const PACKAGE: &'static str = "bilibili.app.viewunite.pgcanymodel";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.viewunite.pgcanymodel.UserStatus".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.viewunite.pgcanymodel.UserStatus".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewPgcAny {
    ///
    #[prost(message, optional, tag = "1")]
    pub ogv_data: ::core::option::Option<OgvData>,
    ///
    #[prost(map = "int64, message", tag = "2")]
    pub all_up_info: ::std::collections::HashMap<i64, super::common::Staff>,
}
impl ::prost::Name for ViewPgcAny {
    const NAME: &'static str = "ViewPgcAny";
    const PACKAGE: &'static str = "bilibili.app.viewunite.pgcanymodel";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.viewunite.pgcanymodel.ViewPgcAny".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.viewunite.pgcanymodel.ViewPgcAny".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WatchProgress {
    ///
    #[prost(int64, tag = "1")]
    pub last_ep_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub last_ep_index: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub last_time: i64,
}
impl ::prost::Name for WatchProgress {
    const NAME: &'static str = "WatchProgress";
    const PACKAGE: &'static str = "bilibili.app.viewunite.pgcanymodel";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.viewunite.pgcanymodel.WatchProgress".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.viewunite.pgcanymodel.WatchProgress".into()
    }
}
