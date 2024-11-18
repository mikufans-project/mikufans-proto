// This file is @generated by prost-build.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssistantInfo {
    ///
    #[prost(int64, tag = "1")]
    pub rank: i64,
    ///
    #[prost(int64, tag = "2")]
    pub uid: i64,
    ///
    #[prost(string, tag = "3")]
    pub uname: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub face: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub award_content: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "6")]
    pub is_mystery: bool,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DmConf {
    ///
    #[prost(string, tag = "1")]
    pub font_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub bg_color: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EscapeInfo {
    ///
    #[prost(int64, tag = "1")]
    pub count: i64,
    ///
    #[prost(int64, tag = "2")]
    pub puni_time: i64,
    ///
    #[prost(string, tag = "3")]
    pub tips: ::prost::alloc::string::String,
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct FinalConf {
    ///
    #[prost(int64, tag = "1")]
    pub start_time: i64,
    ///
    #[prost(int64, tag = "2")]
    pub end_time: i64,
    ///
    #[prost(int64, tag = "3")]
    pub switch: i64,
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct InvitePkResp {
    ///
    #[prost(int64, tag = "1")]
    pub invited_id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub wait_time: i64,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PkScoreMultiplePlay {
    ///
    #[prost(int64, tag = "1")]
    pub pk_id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub status: i64,
    ///
    #[prost(int64, tag = "3")]
    pub target_votes: i64,
    ///
    #[prost(int64, tag = "4")]
    pub multiple_collect_start_time: i64,
    ///
    #[prost(int64, tag = "5")]
    pub multiple_collect_end_time: i64,
    ///
    #[prost(string, tag = "6")]
    pub award_name: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "7")]
    pub award_num: i64,
    ///
    #[prost(int64, tag = "8")]
    pub award_no: i64,
    ///
    #[prost(string, tag = "9")]
    pub rule_url: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "10")]
    pub draw_start_time: i64,
    ///
    #[prost(int64, tag = "11")]
    pub draw_end_time: i64,
    ///
    #[prost(string, tag = "12")]
    pub draw_award_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "13")]
    pub draw_award_value: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "14")]
    pub award_multiple_time: i64,
    ///
    #[prost(int64, tag = "15")]
    pub award_start_time: i64,
    ///
    #[prost(int64, tag = "16")]
    pub award_end_time: i64,
    ///
    #[prost(int64, tag = "17")]
    pub power: i64,
    ///
    #[prost(string, tag = "18")]
    pub guide_str: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PkCard {
    ///
    #[prost(string, tag = "1")]
    pub card_id: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub card_type: i64,
    ///
    #[prost(int64, tag = "3")]
    pub end_time: i64,
    ///
    #[prost(string, tag = "4")]
    pub text: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PkCardPlay {
    ///
    #[prost(string, tag = "1")]
    pub rule_url: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PkInfo {
    ///
    #[prost(message, optional, tag = "1")]
    pub pk_basic: ::core::option::Option<PkBasic>,
    ///
    #[prost(message, repeated, tag = "2")]
    pub members: ::prost::alloc::vec::Vec<PkUser>,
    ///
    #[prost(int64, tag = "3")]
    pub timestamp: i64,
    ///
    #[prost(int64, tag = "4")]
    pub mill_timestamp: i64,
    ///
    #[prost(message, repeated, tag = "5")]
    pub pk_group: ::prost::alloc::vec::Vec<PkGroup>,
    ///
    #[prost(message, optional, tag = "6")]
    pub pk_match_info: ::core::option::Option<PkMatchInfo>,
    ///
    #[prost(message, optional, tag = "7")]
    pub invite_pk_resp: ::core::option::Option<InvitePkResp>,
    ///
    #[prost(message, optional, tag = "8")]
    pub pk_play: ::core::option::Option<PkPlay>,
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PkMatchInfo {
    ///
    #[prost(int64, tag = "1")]
    pub match_status: i64,
    ///
    #[prost(int64, tag = "2")]
    pub match_max_time: i64,
    ///
    #[prost(int64, tag = "3")]
    pub match_end_time: i64,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PkPlay {
    ///
    #[prost(message, optional, tag = "1")]
    pub pk_score_multiple_play: ::core::option::Option<PkScoreMultiplePlay>,
    ///
    #[prost(message, optional, tag = "2")]
    pub final_conf: ::core::option::Option<FinalConf>,
    ///
    #[prost(bool, tag = "3")]
    pub show_streak: bool,
    ///
    #[prost(message, optional, tag = "4")]
    pub dm_conf: ::core::option::Option<DmConf>,
    ///
    #[prost(message, optional, tag = "5")]
    pub escape: ::core::option::Option<EscapeInfo>,
    ///
    #[prost(message, optional, tag = "6")]
    pub pk_card_play: ::core::option::Option<PkCardPlay>,
    ///
    #[prost(int64, tag = "7")]
    pub pre_duration: i64,
    ///
    #[prost(string, tag = "8")]
    pub pk_play_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub pk_punish_text: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "10")]
    pub pk_text_hash: i64,
    ///
    #[prost(bool, tag = "11")]
    pub pk_text_enabled: bool,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PkBasic {
    ///
    #[prost(int64, tag = "1")]
    pub pk_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub biz_session_id: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub init_id: i64,
    ///
    #[prost(int64, tag = "4")]
    pub init_uid: i64,
    ///
    #[prost(int64, tag = "5")]
    pub status: i64,
    ///
    #[prost(int64, tag = "6")]
    pub r#type: i64,
    ///
    #[prost(int64, tag = "7")]
    pub sub_type: i64,
    ///
    #[prost(int64, tag = "8")]
    pub start_time: i64,
    ///
    #[prost(int64, tag = "9")]
    pub end_time: i64,
    ///
    #[prost(int64, tag = "10")]
    pub punish_end_time: i64,
    ///
    #[prost(int64, tag = "11")]
    pub sprint_duration: i64,
    ///
    #[prost(string, tag = "12")]
    pub punish_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "13")]
    pub template_id: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "14")]
    pub main_page: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "15")]
    pub muti_pk_type: i64,
    ///
    #[prost(int64, tag = "16")]
    pub season_id: i64,
    ///
    #[prost(string, tag = "17")]
    pub status_msg: ::prost::alloc::string::String,
    ///
    #[prost(map = "string, string", tag = "18")]
    pub satellite_info: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PkGroup {
    ///
    #[prost(int64, tag = "1")]
    pub group_id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub votes: i64,
    ///
    #[prost(string, tag = "3")]
    pub votes_text: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "4")]
    pub is_winner: i64,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PkUser {
    ///
    #[prost(int64, tag = "1")]
    pub uid: i64,
    ///
    #[prost(int64, tag = "2")]
    pub room_id: i64,
    ///
    #[prost(string, tag = "3")]
    pub uname: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub face: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub votes: i64,
    ///
    #[prost(string, tag = "6")]
    pub votes_text: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "7")]
    pub rank: i64,
    ///
    #[prost(int64, tag = "8")]
    pub status: i64,
    ///
    #[prost(message, repeated, tag = "9")]
    pub assist_info: ::prost::alloc::vec::Vec<AssistantInfo>,
    ///
    #[prost(int64, tag = "10")]
    pub is_follow: i64,
    ///
    #[prost(int64, tag = "11")]
    pub is_winner: i64,
    ///
    #[prost(int64, tag = "12")]
    pub group_id: i64,
    ///
    #[prost(int64, tag = "13")]
    pub golds: i64,
    ///
    #[prost(int64, tag = "14")]
    pub date_streak: i64,
    ///
    #[prost(int64, tag = "15")]
    pub pk_multiple_status: i64,
    ///
    #[prost(string, tag = "16")]
    pub power: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "17")]
    pub is_latest_streak: bool,
    ///
    #[prost(message, repeated, tag = "19")]
    pub pk_cards: ::prost::alloc::vec::Vec<PkCard>,
}
