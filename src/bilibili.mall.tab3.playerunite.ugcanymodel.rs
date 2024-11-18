// This file is @generated by prost-build.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ButtonStyle {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub text_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub jump_link: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayLimit {
    ///
    #[prost(enumeration = "PlayLimitCode", tag = "1")]
    pub code: i32,
    ///
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub sub_message: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "4")]
    pub button: ::core::option::Option<ButtonStyle>,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UgcAnyModel {
    ///
    #[prost(message, optional, tag = "1")]
    pub play_limit: ::core::option::Option<PlayLimit>,
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PlayLimitCode {
    ///
    PlcUnknown = 0,
    ///
    PlcUgcnotpayed = 1,
    ///
    PlcChargingPlusNotPass = 2,
    ///
    PlcChargingPlusUpgrade = 3,
    ///
    PlcChargingPlusReject = 4,
}
impl PlayLimitCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::PlcUnknown => "PLC_UNKNOWN",
            Self::PlcUgcnotpayed => "PLC_UGCNOTPAYED",
            Self::PlcChargingPlusNotPass => "PLC_ChargingPlusNotPass",
            Self::PlcChargingPlusUpgrade => "PLC_ChargingPlusUpgrade",
            Self::PlcChargingPlusReject => "PLC_ChargingPlusReject",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PLC_UNKNOWN" => Some(Self::PlcUnknown),
            "PLC_UGCNOTPAYED" => Some(Self::PlcUgcnotpayed),
            "PLC_ChargingPlusNotPass" => Some(Self::PlcChargingPlusNotPass),
            "PLC_ChargingPlusUpgrade" => Some(Self::PlcChargingPlusUpgrade),
            "PLC_ChargingPlusReject" => Some(Self::PlcChargingPlusReject),
            _ => None,
        }
    }
}
