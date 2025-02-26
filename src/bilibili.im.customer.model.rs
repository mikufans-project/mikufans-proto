// This file is @generated by prost-build.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BindNote {
    ///
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub high_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub color: ::prost::alloc::string::String,
}
impl ::prost::Name for BindNote {
    const NAME: &'static str = "BindNote";
    const PACKAGE: &'static str = "bilibili.im.customer.model";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.im.customer.model.BindNote".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.im.customer.model.BindNote".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Business {
    ///
    #[prost(int64, tag = "1")]
    pub business_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub business_name: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "Select", tag = "3")]
    pub select: i32,
}
impl ::prost::Name for Business {
    const NAME: &'static str = "Business";
    const PACKAGE: &'static str = "bilibili.im.customer.model";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.im.customer.model.Business".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.im.customer.model.Business".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComplianceDescribe {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "2")]
    pub is_link: bool,
    ///
    #[prost(string, tag = "3")]
    pub link: ::prost::alloc::string::String,
}
impl ::prost::Name for ComplianceDescribe {
    const NAME: &'static str = "ComplianceDescribe";
    const PACKAGE: &'static str = "bilibili.im.customer.model";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.im.customer.model.ComplianceDescribe".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.im.customer.model.ComplianceDescribe".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComplianceModel {
    ///
    #[prost(enumeration = "ComplianceAlertType", tag = "1")]
    pub pop_field: i32,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "3")]
    pub describes: ::prost::alloc::vec::Vec<ComplianceDescribe>,
    ///
    #[prost(string, tag = "4")]
    pub confirm_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub cancel_text: ::prost::alloc::string::String,
}
impl ::prost::Name for ComplianceModel {
    const NAME: &'static str = "ComplianceModel";
    const PACKAGE: &'static str = "bilibili.im.customer.model";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.im.customer.model.ComplianceModel".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.im.customer.model.ComplianceModel".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerInfo {
    ///
    #[prost(int64, tag = "1")]
    pub customer_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub customer_name: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "CustomerRankStatus", tag = "3")]
    pub customer_state: i32,
    ///
    #[prost(int64, tag = "4")]
    pub queue_rank: i64,
    ///
    #[prost(int32, tag = "5")]
    pub is_cancel: i32,
    ///
    #[prost(message, optional, tag = "6")]
    pub pre_customer: ::core::option::Option<MsgSpLitAnsCustomer>,
    ///
    #[prost(string, tag = "7")]
    pub customer_state_desc: ::prost::alloc::string::String,
}
impl ::prost::Name for CustomerInfo {
    const NAME: &'static str = "CustomerInfo";
    const PACKAGE: &'static str = "bilibili.im.customer.model";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.im.customer.model.CustomerInfo".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.im.customer.model.CustomerInfo".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmotionInfo {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "3")]
    pub size: i32,
    ///
    #[prost(string, tag = "4")]
    pub gif_url: ::prost::alloc::string::String,
}
impl ::prost::Name for EmotionInfo {
    const NAME: &'static str = "EmotionInfo";
    const PACKAGE: &'static str = "bilibili.im.customer.model";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.im.customer.model.EmotionInfo".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.im.customer.model.EmotionInfo".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluationShowInfo {
    ///
    #[prost(bool, tag = "1")]
    pub show: bool,
    ///
    #[prost(enumeration = "EvaluateType", tag = "2")]
    pub r#type: i32,
    ///
    #[prost(message, repeated, tag = "3")]
    pub step_select: ::prost::alloc::vec::Vec<EvaluationStepSelect>,
    ///
    #[prost(int64, tag = "4")]
    pub msg_key: i64,
    ///
    #[prost(bool, tag = "5")]
    pub up: bool,
    ///
    #[prost(string, tag = "6")]
    pub feedback: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "7")]
    pub customer_sess_id: i64,
    ///
    #[prost(bool, tag = "8")]
    pub done: bool,
    ///
    #[prost(string, tag = "9")]
    pub editor_input: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "10")]
    pub step_msg_key: i64,
    ///
    #[prost(int64, tag = "11")]
    pub machine_sess_id: i64,
}
impl ::prost::Name for EvaluationShowInfo {
    const NAME: &'static str = "EvaluationShowInfo";
    const PACKAGE: &'static str = "bilibili.im.customer.model";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.im.customer.model.EvaluationShowInfo".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.im.customer.model.EvaluationShowInfo".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluationStepSelect {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "Select", tag = "2")]
    pub select: i32,
}
impl ::prost::Name for EvaluationStepSelect {
    const NAME: &'static str = "EvaluationStepSelect";
    const PACKAGE: &'static str = "bilibili.im.customer.model";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.im.customer.model.EvaluationStepSelect".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.im.customer.model.EvaluationStepSelect".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Group {
    ///
    #[prost(int64, tag = "1")]
    pub group_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub group_name: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "Select", tag = "3")]
    pub select: i32,
}
impl ::prost::Name for Group {
    const NAME: &'static str = "Group";
    const PACKAGE: &'static str = "bilibili.im.customer.model";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.im.customer.model.Group".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.im.customer.model.Group".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Msg {
    ///
    #[prost(message, optional, tag = "1")]
    pub sender_info: ::core::option::Option<TalkerInfo>,
    ///
    #[prost(message, optional, tag = "2")]
    pub receiver_info: ::core::option::Option<TalkerInfo>,
    ///
    #[prost(enumeration = "MsgType", tag = "3")]
    pub msg_type: i32,
    ///
    #[prost(string, tag = "4")]
    pub content: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub timestamp: i64,
    ///
    #[prost(int64, tag = "6")]
    pub msg_key: i64,
    ///
    #[prost(int32, tag = "7")]
    pub msg_status: i32,
    ///
    #[prost(enumeration = "MsgSource", tag = "8")]
    pub msg_source: i32,
    ///
    #[prost(string, tag = "9")]
    pub dev_id: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "10")]
    pub seq_no: i64,
    ///
    #[prost(int32, tag = "11")]
    pub situation: i32,
    ///
    #[prost(message, optional, tag = "12")]
    pub bind_note: ::core::option::Option<BindNote>,
    ///
    #[prost(bool, tag = "13")]
    pub is_ai: bool,
    ///
    #[prost(int64, repeated, tag = "14")]
    pub ai_link_msg: ::prost::alloc::vec::Vec<i64>,
}
impl ::prost::Name for Msg {
    const NAME: &'static str = "Msg";
    const PACKAGE: &'static str = "bilibili.im.customer.model";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.im.customer.model.Msg".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.im.customer.model.Msg".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSpLitA {
    ///
    #[prost(enumeration = "SplitAnsMsgContentType", tag = "1")]
    pub r#type: i32,
    ///
    #[prost(message, repeated, tag = "2")]
    pub business_list: ::prost::alloc::vec::Vec<Business>,
    ///
    #[prost(message, repeated, tag = "3")]
    pub group_list: ::prost::alloc::vec::Vec<Group>,
    ///
    #[prost(message, optional, tag = "4")]
    pub customer_info: ::core::option::Option<CustomerInfo>,
}
impl ::prost::Name for MsgSpLitA {
    const NAME: &'static str = "MsgSpLitA";
    const PACKAGE: &'static str = "bilibili.im.customer.model";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.im.customer.model.MsgSpLitA".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.im.customer.model.MsgSpLitA".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSpLitAnsCustomer {
    ///
    #[prost(int64, tag = "1")]
    pub customer_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub customer_name: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgSpLitAnsCustomer {
    const NAME: &'static str = "MsgSpLitAnsCustomer";
    const PACKAGE: &'static str = "bilibili.im.customer.model";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.im.customer.model.MsgSpLitAnsCustomer".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.im.customer.model.MsgSpLitAnsCustomer".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionInfo {
    ///
    #[prost(message, optional, tag = "1")]
    pub talker_info: ::core::option::Option<TalkerInfo>,
    ///
    #[prost(message, optional, tag = "2")]
    pub last_msg: ::core::option::Option<Msg>,
    ///
    #[prost(message, optional, tag = "3")]
    pub unread_info: ::core::option::Option<UnreadInfo>,
    ///
    #[prost(int64, tag = "4")]
    pub ack_seqno: i64,
    ///
    #[prost(int64, tag = "5")]
    pub ack_ts: i64,
    ///
    #[prost(int64, tag = "6")]
    pub session_ts: i64,
    ///
    #[prost(int64, tag = "7")]
    pub max_seqno: i64,
    ///
    #[prost(int32, tag = "8")]
    pub status: i32,
    ///
    #[prost(string, tag = "9")]
    pub tag_icon: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "10")]
    pub top_set: i32,
    ///
    #[prost(string, tag = "11")]
    pub jump_url: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "12")]
    pub do_not_disturb: bool,
    ///
    #[prost(string, tag = "13")]
    pub setting_page_schema: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "14")]
    pub setting_page_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "15")]
    pub tag_name: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "16")]
    pub compliance: ::core::option::Option<ComplianceModel>,
    ///
    #[prost(bool, tag = "17")]
    pub is_hide_edit: bool,
}
impl ::prost::Name for SessionInfo {
    const NAME: &'static str = "SessionInfo";
    const PACKAGE: &'static str = "bilibili.im.customer.model";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.im.customer.model.SessionInfo".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.im.customer.model.SessionInfo".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TalkerInfo {
    ///
    #[prost(enumeration = "TalkerType", tag = "1")]
    pub r#type: i32,
    ///
    #[prost(int64, tag = "2")]
    pub shop_father_id: i64,
    ///
    #[prost(int64, tag = "3")]
    pub shop_id: i64,
    ///
    #[prost(int64, tag = "4")]
    pub customer_id: i64,
    ///
    #[prost(int64, tag = "5")]
    pub machine_session_id: i64,
    ///
    #[prost(string, tag = "6")]
    pub customer_session_id: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "7")]
    pub uid: i64,
    ///
    #[prost(string, tag = "8")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub pic_url: ::prost::alloc::string::String,
    ///
    #[prost(string, repeated, tag = "10")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for TalkerInfo {
    const NAME: &'static str = "TalkerInfo";
    const PACKAGE: &'static str = "bilibili.im.customer.model";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.im.customer.model.TalkerInfo".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.im.customer.model.TalkerInfo".into()
    }
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct UnreadInfo {
    ///
    #[prost(int64, tag = "1")]
    pub unread_count: i64,
    ///
    #[prost(int64, tag = "2")]
    pub customer_unread_count: i64,
    ///
    #[prost(int64, tag = "3")]
    pub machine_unread_count: i64,
    ///
    #[prost(int64, tag = "4")]
    pub message_unread_count: i64,
    ///
    #[prost(int64, tag = "5")]
    pub notice_unread_count: i64,
}
impl ::prost::Name for UnreadInfo {
    const NAME: &'static str = "UnreadInfo";
    const PACKAGE: &'static str = "bilibili.im.customer.model";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.im.customer.model.UnreadInfo".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.im.customer.model.UnreadInfo".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WindowOptionEvaluation {
    ///
    #[prost(string, tag = "1")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub jump_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub im_stat: i64,
}
impl ::prost::Name for WindowOptionEvaluation {
    const NAME: &'static str = "WindowOptionEvaluation";
    const PACKAGE: &'static str = "bilibili.im.customer.model";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.im.customer.model.WindowOptionEvaluation".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.im.customer.model.WindowOptionEvaluation".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WindowOptionGoods {
    ///
    #[prost(string, tag = "1")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub jump_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub title: ::prost::alloc::string::String,
}
impl ::prost::Name for WindowOptionGoods {
    const NAME: &'static str = "WindowOptionGoods";
    const PACKAGE: &'static str = "bilibili.im.customer.model";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.im.customer.model.WindowOptionGoods".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.im.customer.model.WindowOptionGoods".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WindowOptionHistory {
    ///
    #[prost(string, tag = "1")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub jump_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub title: ::prost::alloc::string::String,
}
impl ::prost::Name for WindowOptionHistory {
    const NAME: &'static str = "WindowOptionHistory";
    const PACKAGE: &'static str = "bilibili.im.customer.model";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.im.customer.model.WindowOptionHistory".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.im.customer.model.WindowOptionHistory".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WindowOptionMessage {
    ///
    #[prost(string, tag = "1")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub jump_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub sub_title: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "6")]
    pub is_new_ticket: bool,
    ///
    #[prost(string, tag = "7")]
    pub new_ticket_jump_url: ::prost::alloc::string::String,
}
impl ::prost::Name for WindowOptionMessage {
    const NAME: &'static str = "WindowOptionMessage";
    const PACKAGE: &'static str = "bilibili.im.customer.model";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.im.customer.model.WindowOptionMessage".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.im.customer.model.WindowOptionMessage".into()
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WindowOptionOrder {
    ///
    #[prost(string, tag = "1")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub jump_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub title: ::prost::alloc::string::String,
}
impl ::prost::Name for WindowOptionOrder {
    const NAME: &'static str = "WindowOptionOrder";
    const PACKAGE: &'static str = "bilibili.im.customer.model";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.im.customer.model.WindowOptionOrder".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.im.customer.model.WindowOptionOrder".into()
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ComplianceAlertType {
    ///
    None = 0,
    ///
    AiModel = 1,
}
impl ComplianceAlertType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::AiModel => "AIModel",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "None" => Some(Self::None),
            "AIModel" => Some(Self::AiModel),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CustomerRankStatus {
    ///
    Offline = 0,
    ///
    Crowd = 1,
    ///
    Online = 2,
    ///
    SwitchOnline = 3,
    ///
    Ranking = 4,
    ///
    SwitchRanking = 5,
    ///
    OutCustomerQueue = 6,
    ///
    OutCustomerQueueWithMessage = 7,
    ///
    OutWork = 8,
    ///
    CustomerInvite = 9,
    ///
    WorkBatchStopRank = 10,
    ///
    ForceSwitchCustomer = 11,
    ///
    RankingOnly = 12,
    ///
    NoCustomerOnly = 13,
    ///
    OfflineProcess = 14,
}
impl CustomerRankStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Offline => "OFFLINE",
            Self::Crowd => "CROWD",
            Self::Online => "ONLINE",
            Self::SwitchOnline => "SWITCH_ONLINE",
            Self::Ranking => "RANKING",
            Self::SwitchRanking => "SWITCH_RANKING",
            Self::OutCustomerQueue => "OUT_CUSTOMER_QUEUE",
            Self::OutCustomerQueueWithMessage => "OUT_CUSTOMER_QUEUE_WITH_MESSAGE",
            Self::OutWork => "OUT_WORK",
            Self::CustomerInvite => "CUSTOMER_INVITE",
            Self::WorkBatchStopRank => "WORK_BATCH_STOP_RANK",
            Self::ForceSwitchCustomer => "FORCE_SWITCH_CUSTOMER",
            Self::RankingOnly => "RANKING_ONLY",
            Self::NoCustomerOnly => "NO_CUSTOMER_ONLY",
            Self::OfflineProcess => "OFFLINE_PROCESS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OFFLINE" => Some(Self::Offline),
            "CROWD" => Some(Self::Crowd),
            "ONLINE" => Some(Self::Online),
            "SWITCH_ONLINE" => Some(Self::SwitchOnline),
            "RANKING" => Some(Self::Ranking),
            "SWITCH_RANKING" => Some(Self::SwitchRanking),
            "OUT_CUSTOMER_QUEUE" => Some(Self::OutCustomerQueue),
            "OUT_CUSTOMER_QUEUE_WITH_MESSAGE" => Some(Self::OutCustomerQueueWithMessage),
            "OUT_WORK" => Some(Self::OutWork),
            "CUSTOMER_INVITE" => Some(Self::CustomerInvite),
            "WORK_BATCH_STOP_RANK" => Some(Self::WorkBatchStopRank),
            "FORCE_SWITCH_CUSTOMER" => Some(Self::ForceSwitchCustomer),
            "RANKING_ONLY" => Some(Self::RankingOnly),
            "NO_CUSTOMER_ONLY" => Some(Self::NoCustomerOnly),
            "OFFLINE_PROCESS" => Some(Self::OfflineProcess),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EvaluateType {
    ///
    EvaluateMachineMsg = 0,
    ///
    EvaluateCustomer = 1,
    ///
    EvaluateCustomerInvited = 2,
    ///
    EvaluatePanelMsg = 3,
    ///
    EvaluateMachineSess = 4,
    ///
    EvaluateMachineSessInvited = 5,
}
impl EvaluateType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::EvaluateMachineMsg => "EvaluateMachineMsg",
            Self::EvaluateCustomer => "EvaluateCustomer",
            Self::EvaluateCustomerInvited => "EvaluateCustomerInvited",
            Self::EvaluatePanelMsg => "EvaluatePanelMsg",
            Self::EvaluateMachineSess => "EvaluateMachineSess",
            Self::EvaluateMachineSessInvited => "EvaluateMachineSessInvited",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EvaluateMachineMsg" => Some(Self::EvaluateMachineMsg),
            "EvaluateCustomer" => Some(Self::EvaluateCustomer),
            "EvaluateCustomerInvited" => Some(Self::EvaluateCustomerInvited),
            "EvaluatePanelMsg" => Some(Self::EvaluatePanelMsg),
            "EvaluateMachineSess" => Some(Self::EvaluateMachineSess),
            "EvaluateMachineSessInvited" => Some(Self::EvaluateMachineSessInvited),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MsgSource {
    ///
    Invalid = 0,
    ///
    Ios = 1,
    ///
    Android = 2,
    ///
    Ipad = 3,
    ///
    AndroidHd = 4,
    ///
    PcApp = 5,
    ///
    Web = 6,
    ///
    Biz = 7,
    ///
    ThirdShopStage = 8,
    ///
    Ai = 9,
}
impl MsgSource {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Invalid => "INVALID",
            Self::Ios => "IOS",
            Self::Android => "ANDROID",
            Self::Ipad => "IPAD",
            Self::AndroidHd => "ANDROID_HD",
            Self::PcApp => "PC_APP",
            Self::Web => "WEB",
            Self::Biz => "Biz",
            Self::ThirdShopStage => "ThirdShopStage",
            Self::Ai => "AI",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID" => Some(Self::Invalid),
            "IOS" => Some(Self::Ios),
            "ANDROID" => Some(Self::Android),
            "IPAD" => Some(Self::Ipad),
            "ANDROID_HD" => Some(Self::AndroidHd),
            "PC_APP" => Some(Self::PcApp),
            "WEB" => Some(Self::Web),
            "Biz" => Some(Self::Biz),
            "ThirdShopStage" => Some(Self::ThirdShopStage),
            "AI" => Some(Self::Ai),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MsgType {
    ///
    Invalid = 0,
    ///
    Text = 1,
    ///
    Img = 2,
    ///
    Withdraw = 5,
    ///
    FavEmoji = 6,
    ///
    Share = 7,
    ///
    Video = 17,
    ///
    Order = 10001,
    ///
    Goods = 10002,
    ///
    Evaluation = 10003,
    ///
    Message = 10004,
    ///
    MachineA = 10005,
    ///
    MachineQ = 10006,
    ///
    CustomerCome = 10007,
    ///
    MachineWelcomeText = 10008,
    ///
    MachineWelcomeQlist = 10009,
    ///
    SplitQ = 10010,
    ///
    SplitA = 10011,
    ///
    SwitchCustomer = 10012,
    ///
    CustomerInviteEvaluation = 10013,
    ///
    OutCustomerQueue = 10014,
    ///
    OutCustomerSess = 10015,
    ///
    MessageGuide = 10016,
    ///
    ForceSwitchCustomer = 10017,
    ///
    CustomerInitiativeMsg = 10018,
    ///
    SysNote = 10019,
    ///
    MachineSessEvaluation = 10020,
    ///
    ParagraphText = 10021,
    ///
    BusinessLink = 10022,
    ///
    SysBar = 10023,
    ///
    VirtualCome = 10024,
    ///
    CloseVirtualCome = 10025,
    ///
    CommentQuote = 10026,
}
impl MsgType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Invalid => "MSG_TYPE_INVALID",
            Self::Text => "MSG_TYPE_TEXT",
            Self::Img => "MSG_TYPE_IMG",
            Self::Withdraw => "MSG_TYPE_WITHDRAW",
            Self::FavEmoji => "MSG_TYPE_FAV_EMOJI",
            Self::Share => "MSG_TYPE_SHARE",
            Self::Video => "MSG_TYPE_VIDEO",
            Self::Order => "MSG_TYPE_ORDER",
            Self::Goods => "MSG_TYPE_GOODS",
            Self::Evaluation => "MSG_TYPE_EVALUATION",
            Self::Message => "MSG_TYPE_MESSAGE",
            Self::MachineA => "MSG_TYPE_MACHINE_A",
            Self::MachineQ => "MSG_TYPE_MACHINE_Q",
            Self::CustomerCome => "MSG_TYPE_CUSTOMER_COME",
            Self::MachineWelcomeText => "MSG_TYPE_MACHINE_WELCOME_TEXT",
            Self::MachineWelcomeQlist => "MSG_TYPE_MACHINE_WELCOME_QLIST",
            Self::SplitQ => "MSG_TYPE_SPLIT_Q",
            Self::SplitA => "MSG_TYPE_SPLIT_A",
            Self::SwitchCustomer => "MSG_TYPE_SWITCH_CUSTOMER",
            Self::CustomerInviteEvaluation => "MSG_TYPE_CUSTOMER_INVITE_EVALUATION",
            Self::OutCustomerQueue => "MSG_TYPE_OUT_CUSTOMER_QUEUE",
            Self::OutCustomerSess => "MSG_TYPE_OUT_CUSTOMER_SESS",
            Self::MessageGuide => "MSG_TYPE_MESSAGE_GUIDE",
            Self::ForceSwitchCustomer => "MSG_TYPE_FORCE_SWITCH_CUSTOMER",
            Self::CustomerInitiativeMsg => "MSG_TYPE_CUSTOMER_INITIATIVE_MSG",
            Self::SysNote => "MSG_TYPE_SYS_NOTE",
            Self::MachineSessEvaluation => "MSG_TYPE_MACHINE_SESS_EVALUATION",
            Self::ParagraphText => "MSG_TYPE_PARAGRAPH_TEXT",
            Self::BusinessLink => "MSG_TYPE_BUSINESS_LINK",
            Self::SysBar => "MSG_TYPE_SYS_BAR",
            Self::VirtualCome => "MSG_TYPE_VIRTUAL_COME",
            Self::CloseVirtualCome => "MSG_TYPE_CLOSE_VIRTUAL_COME",
            Self::CommentQuote => "MSG_TYPE_COMMENT_QUOTE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MSG_TYPE_INVALID" => Some(Self::Invalid),
            "MSG_TYPE_TEXT" => Some(Self::Text),
            "MSG_TYPE_IMG" => Some(Self::Img),
            "MSG_TYPE_WITHDRAW" => Some(Self::Withdraw),
            "MSG_TYPE_FAV_EMOJI" => Some(Self::FavEmoji),
            "MSG_TYPE_SHARE" => Some(Self::Share),
            "MSG_TYPE_VIDEO" => Some(Self::Video),
            "MSG_TYPE_ORDER" => Some(Self::Order),
            "MSG_TYPE_GOODS" => Some(Self::Goods),
            "MSG_TYPE_EVALUATION" => Some(Self::Evaluation),
            "MSG_TYPE_MESSAGE" => Some(Self::Message),
            "MSG_TYPE_MACHINE_A" => Some(Self::MachineA),
            "MSG_TYPE_MACHINE_Q" => Some(Self::MachineQ),
            "MSG_TYPE_CUSTOMER_COME" => Some(Self::CustomerCome),
            "MSG_TYPE_MACHINE_WELCOME_TEXT" => Some(Self::MachineWelcomeText),
            "MSG_TYPE_MACHINE_WELCOME_QLIST" => Some(Self::MachineWelcomeQlist),
            "MSG_TYPE_SPLIT_Q" => Some(Self::SplitQ),
            "MSG_TYPE_SPLIT_A" => Some(Self::SplitA),
            "MSG_TYPE_SWITCH_CUSTOMER" => Some(Self::SwitchCustomer),
            "MSG_TYPE_CUSTOMER_INVITE_EVALUATION" => Some(Self::CustomerInviteEvaluation),
            "MSG_TYPE_OUT_CUSTOMER_QUEUE" => Some(Self::OutCustomerQueue),
            "MSG_TYPE_OUT_CUSTOMER_SESS" => Some(Self::OutCustomerSess),
            "MSG_TYPE_MESSAGE_GUIDE" => Some(Self::MessageGuide),
            "MSG_TYPE_FORCE_SWITCH_CUSTOMER" => Some(Self::ForceSwitchCustomer),
            "MSG_TYPE_CUSTOMER_INITIATIVE_MSG" => Some(Self::CustomerInitiativeMsg),
            "MSG_TYPE_SYS_NOTE" => Some(Self::SysNote),
            "MSG_TYPE_MACHINE_SESS_EVALUATION" => Some(Self::MachineSessEvaluation),
            "MSG_TYPE_PARAGRAPH_TEXT" => Some(Self::ParagraphText),
            "MSG_TYPE_BUSINESS_LINK" => Some(Self::BusinessLink),
            "MSG_TYPE_SYS_BAR" => Some(Self::SysBar),
            "MSG_TYPE_VIRTUAL_COME" => Some(Self::VirtualCome),
            "MSG_TYPE_CLOSE_VIRTUAL_COME" => Some(Self::CloseVirtualCome),
            "MSG_TYPE_COMMENT_QUOTE" => Some(Self::CommentQuote),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NotifyMsgType {
    ///
    UnUsable = 0,
    ///
    RankUpdate = 400,
    ///
    MsgTalk = 401,
    ///
    SubmitWorkOrderDraft = 402,
}
impl NotifyMsgType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::UnUsable => "UN_USABLE",
            Self::RankUpdate => "RANK_UPDATE",
            Self::MsgTalk => "MSG_TALK",
            Self::SubmitWorkOrderDraft => "SUBMIT_WORK_ORDER_DRAFT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UN_USABLE" => Some(Self::UnUsable),
            "RANK_UPDATE" => Some(Self::RankUpdate),
            "MSG_TALK" => Some(Self::MsgTalk),
            "SUBMIT_WORK_ORDER_DRAFT" => Some(Self::SubmitWorkOrderDraft),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Select {
    ///
    Able = 0,
    ///
    Selected = 1,
    ///
    Unable = 2,
}
impl Select {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Able => "ABLE",
            Self::Selected => "SELECTED",
            Self::Unable => "UNABLE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ABLE" => Some(Self::Able),
            "SELECTED" => Some(Self::Selected),
            "UNABLE" => Some(Self::Unable),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Situation {
    ///
    Invalid = 0,
    ///
    Im = 1,
    ///
    ServiceCenter = 2,
    ///
    Order = 3,
    ///
    Push = 4,
    ///
    Goods = 5,
    ///
    VipMall = 6,
    ///
    Else = 7,
    ///
    Live = 8,
    ///
    Space = 9,
}
impl Situation {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Invalid => "Invalid",
            Self::Im => "IM",
            Self::ServiceCenter => "ServiceCenter",
            Self::Order => "Order",
            Self::Push => "Push",
            Self::Goods => "Goods",
            Self::VipMall => "VipMall",
            Self::Else => "Else",
            Self::Live => "Live",
            Self::Space => "Space",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Invalid" => Some(Self::Invalid),
            "IM" => Some(Self::Im),
            "ServiceCenter" => Some(Self::ServiceCenter),
            "Order" => Some(Self::Order),
            "Push" => Some(Self::Push),
            "Goods" => Some(Self::Goods),
            "VipMall" => Some(Self::VipMall),
            "Else" => Some(Self::Else),
            "Live" => Some(Self::Live),
            "Space" => Some(Self::Space),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SplitAnsMsgContentType {
    ///
    Business = 0,
    ///
    Group = 1,
    ///
    Custom = 2,
}
impl SplitAnsMsgContentType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Business => "BUSINESS",
            Self::Group => "GROUP",
            Self::Custom => "CUSTOM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BUSINESS" => Some(Self::Business),
            "GROUP" => Some(Self::Group),
            "CUSTOM" => Some(Self::Custom),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TalkerType {
    ///
    Machine = 0,
    ///
    Customer = 1,
    ///
    SysNotice = 2,
    ///
    User = 3,
}
impl TalkerType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Machine => "MACHINE",
            Self::Customer => "CUSTOMER",
            Self::SysNotice => "SYS_NOTICE",
            Self::User => "USER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MACHINE" => Some(Self::Machine),
            "CUSTOMER" => Some(Self::Customer),
            "SYS_NOTICE" => Some(Self::SysNotice),
            "USER" => Some(Self::User),
            _ => None,
        }
    }
}
