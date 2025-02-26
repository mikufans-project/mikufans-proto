// This file is @generated by prost-build.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadSettingsConfig {
    ///
    #[prost(message, optional, tag = "1")]
    pub enable_download_auto_start: ::core::option::Option<super::super::BoolValue>,
}
impl ::prost::Name for DownloadSettingsConfig {
    const NAME: &'static str = "DownloadSettingsConfig";
    const PACKAGE: &'static str = "bilibili.app.distribution.setting.download";
    fn full_name() -> ::prost::alloc::string::String {
        "bilibili.app.distribution.setting.download.DownloadSettingsConfig".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/bilibili.app.distribution.setting.download.DownloadSettingsConfig".into()
    }
}
