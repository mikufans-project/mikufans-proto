#[cfg(feature = "bilibili")]
pub mod bilibili {
    #[cfg(feature = "bilibili_account")]
    pub mod account {
        #[cfg(feature = "bilibili_account_fission")]
        pub mod fission {
            #[cfg(feature = "bilibili_account_fission_v1")]
            pub mod v1 {
                include!("bilibili.account.fission.v1.rs");
            }
        }
        #[cfg(feature = "bilibili_account_interfaces")]
        pub mod interfaces {
            #[cfg(feature = "bilibili_account_interfaces_v1")]
            pub mod v1 {
                include!("bilibili.account.interfaces.v1.rs");
            }
        }
        #[cfg(feature = "bilibili_account_service")]
        pub mod service {
            #[cfg(feature = "bilibili_account_service_v1")]
            pub mod v1 {
                include!("bilibili.account.service.v1.rs");
            }
        }
    }
    #[cfg(feature = "bilibili_ad")]
    pub mod ad {
        #[cfg(feature = "bilibili_ad_v1")]
        pub mod v1 {
            include!("bilibili.ad.v1.rs");
        }
    }
    #[cfg(feature = "bilibili_api")]
    pub mod api {
        #[cfg(feature = "bilibili_api_player")]
        pub mod player {
            #[cfg(feature = "bilibili_api_player_v1")]
            pub mod v1 {
                include!("bilibili.api.player.v1.rs");
            }
        }
        #[cfg(feature = "bilibili_api_probe")]
        pub mod probe {
            #[cfg(feature = "bilibili_api_probe_v1")]
            pub mod v1 {
                include!("bilibili.api.probe.v1.rs");
            }
        }
        #[cfg(feature = "bilibili_api_ticket")]
        pub mod ticket {
            #[cfg(feature = "bilibili_api_ticket_v1")]
            pub mod v1 {
                include!("bilibili.api.ticket.v1.rs");
            }
        }
    }
    #[cfg(feature = "bilibili_app")]
    pub mod app {
        #[cfg(feature = "bilibili_app_archive")]
        pub mod archive {
            #[cfg(feature = "bilibili_app_archive_middleware")]
            pub mod middleware {
                #[cfg(feature = "bilibili_app_archive_middleware_v1")]
                pub mod v1 {
                    include!("bilibili.app.archive.middleware.v1.rs");
                }
            }
            #[cfg(feature = "bilibili_app_archive_v1")]
            pub mod v1 {
                include!("bilibili.app.archive.v1.rs");
            }
        }
        #[cfg(feature = "bilibili_app_card")]
        pub mod card {
            #[cfg(feature = "bilibili_app_card_v1")]
            pub mod v1 {
                include!("bilibili.app.card.v1.rs");
            }
        }
        #[cfg(feature = "bilibili_app_click")]
        pub mod click {
            #[cfg(feature = "bilibili_app_click_v1")]
            pub mod v1 {
                include!("bilibili.app.click.v1.rs");
            }
        }
        #[cfg(feature = "bilibili_app_distribution")]
        pub mod distribution {
            include!("bilibili.app.distribution.rs");
            #[cfg(feature = "bilibili_app_distribution_setting")]
            pub mod setting {
                #[cfg(feature = "bilibili_app_distribution_setting_download")]
                pub mod download {
                    include!("bilibili.app.distribution.setting.download.rs");
                }
                #[cfg(feature = "bilibili_app_distribution_setting_dynamic")]
                pub mod dynamic {
                    include!("bilibili.app.distribution.setting.dynamic.rs");
                }
                #[cfg(feature = "bilibili_app_distribution_setting_experimental")]
                pub mod experimental {
                    include!("bilibili.app.distribution.setting.experimental.rs");
                }
                #[cfg(feature = "bilibili_app_distribution_setting_home")]
                pub mod home {
                    include!("bilibili.app.distribution.setting.home.rs");
                }
                #[cfg(feature = "bilibili_app_distribution_setting_im")]
                pub mod im {
                    include!("bilibili.app.distribution.setting.im.rs");
                }
                #[cfg(feature = "bilibili_app_distribution_setting_internaldevice")]
                pub mod internaldevice {
                    include!("bilibili.app.distribution.setting.internaldevice.rs");
                }
                #[cfg(feature = "bilibili_app_distribution_setting_night")]
                pub mod night {
                    include!("bilibili.app.distribution.setting.night.rs");
                }
                #[cfg(feature = "bilibili_app_distribution_setting_other")]
                pub mod other {
                    include!("bilibili.app.distribution.setting.other.rs");
                }
                #[cfg(feature = "bilibili_app_distribution_setting_pegasus")]
                pub mod pegasus {
                    include!("bilibili.app.distribution.setting.pegasus.rs");
                }
                #[cfg(feature = "bilibili_app_distribution_setting_play")]
                pub mod play {
                    include!("bilibili.app.distribution.setting.play.rs");
                }
                #[cfg(feature = "bilibili_app_distribution_setting_privacy")]
                pub mod privacy {
                    include!("bilibili.app.distribution.setting.privacy.rs");
                }
                #[cfg(feature = "bilibili_app_distribution_setting_search")]
                pub mod search {
                    include!("bilibili.app.distribution.setting.search.rs");
                }
                #[cfg(feature = "bilibili_app_distribution_setting_story")]
                pub mod story {
                    include!("bilibili.app.distribution.setting.story.rs");
                }
            }
        }
        #[cfg(feature = "bilibili_app_dynamic")]
        pub mod dynamic {
            #[cfg(feature = "bilibili_app_dynamic_common")]
            pub mod common {
                include!("bilibili.app.dynamic.common.rs");
            }
            #[cfg(feature = "bilibili_app_dynamic_v1")]
            pub mod v1 {
                include!("bilibili.app.dynamic.v1.rs");
            }
            #[cfg(feature = "bilibili_app_dynamic_v2")]
            pub mod v2 {
                include!("bilibili.app.dynamic.v2.rs");
            }
        }
        #[cfg(feature = "bilibili_app_home")]
        pub mod home {
            #[cfg(feature = "bilibili_app_home_v1")]
            pub mod v1 {
                include!("bilibili.app.home.v1.rs");
            }
        }
        #[cfg(feature = "bilibili_app_im")]
        pub mod im {
            #[cfg(feature = "bilibili_app_im_v1")]
            pub mod v1 {
                include!("bilibili.app.im.v1.rs");
            }
        }
        #[cfg(feature = "bilibili_app_interfaces")]
        pub mod interfaces {
            #[cfg(feature = "bilibili_app_interfaces_v1")]
            pub mod v1 {
                include!("bilibili.app.interfaces.v1.rs");
            }
        }
        #[cfg(feature = "bilibili_app_listener")]
        pub mod listener {
            #[cfg(feature = "bilibili_app_listener_v1")]
            pub mod v1 {
                include!("bilibili.app.listener.v1.rs");
            }
        }
        #[cfg(feature = "bilibili_app_mine")]
        pub mod mine {
            #[cfg(feature = "bilibili_app_mine_v1")]
            pub mod v1 {
                include!("bilibili.app.mine.v1.rs");
            }
        }
        #[cfg(feature = "bilibili_app_playeronline")]
        pub mod playeronline {
            #[cfg(feature = "bilibili_app_playeronline_v1")]
            pub mod v1 {
                include!("bilibili.app.playeronline.v1.rs");
            }
        }
        #[cfg(feature = "bilibili_app_playerunite")]
        pub mod playerunite {
            #[cfg(feature = "bilibili_app_playerunite_pgcanymodel")]
            pub mod pgcanymodel {
                include!("bilibili.app.playerunite.pgcanymodel.rs");
            }
            #[cfg(feature = "bilibili_app_playerunite_pugvanymodel")]
            pub mod pugvanymodel {
                include!("bilibili.app.playerunite.pugvanymodel.rs");
            }
            #[cfg(feature = "bilibili_app_playerunite_ugcanymodel")]
            pub mod ugcanymodel {
                include!("bilibili.app.playerunite.ugcanymodel.rs");
            }
            #[cfg(feature = "bilibili_app_playerunite_v1")]
            pub mod v1 {
                include!("bilibili.app.playerunite.v1.rs");
            }
        }
        #[cfg(feature = "bilibili_app_playurl")]
        pub mod playurl {
            #[cfg(feature = "bilibili_app_playurl_v1")]
            pub mod v1 {
                include!("bilibili.app.playurl.v1.rs");
            }
        }
        #[cfg(feature = "bilibili_app_resource")]
        pub mod resource {
            #[cfg(feature = "bilibili_app_resource_privacy")]
            pub mod privacy {
                #[cfg(feature = "bilibili_app_resource_privacy_v1")]
                pub mod v1 {
                    include!("bilibili.app.resource.privacy.v1.rs");
                }
            }
            #[cfg(feature = "bilibili_app_resource_v1")]
            pub mod v1 {
                include!("bilibili.app.resource.v1.rs");
            }
        }
        #[cfg(feature = "bilibili_app_search")]
        pub mod search {
            #[cfg(feature = "bilibili_app_search_v2")]
            pub mod v2 {
                include!("bilibili.app.search.v2.rs");
            }
        }
        #[cfg(feature = "bilibili_app_show")]
        pub mod show {
            #[cfg(feature = "bilibili_app_show_gateway")]
            pub mod gateway {
                #[cfg(feature = "bilibili_app_show_gateway_v1")]
                pub mod v1 {
                    include!("bilibili.app.show.gateway.v1.rs");
                }
            }
            #[cfg(feature = "bilibili_app_show_mixture")]
            pub mod mixture {
                #[cfg(feature = "bilibili_app_show_mixture_v1")]
                pub mod v1 {
                    include!("bilibili.app.show.mixture.v1.rs");
                }
            }
            #[cfg(feature = "bilibili_app_show_popular")]
            pub mod popular {
                #[cfg(feature = "bilibili_app_show_popular_v1")]
                pub mod v1 {
                    include!("bilibili.app.show.popular.v1.rs");
                }
            }
            #[cfg(feature = "bilibili_app_show_rank")]
            pub mod rank {
                #[cfg(feature = "bilibili_app_show_rank_v1")]
                pub mod v1 {
                    include!("bilibili.app.show.rank.v1.rs");
                }
            }
            #[cfg(feature = "bilibili_app_show_region")]
            pub mod region {
                #[cfg(feature = "bilibili_app_show_region_v1")]
                pub mod v1 {
                    include!("bilibili.app.show.region.v1.rs");
                }
            }
        }
        #[cfg(feature = "bilibili_app_space")]
        pub mod space {
            #[cfg(feature = "bilibili_app_space_v1")]
            pub mod v1 {
                include!("bilibili.app.space.v1.rs");
            }
        }
        #[cfg(feature = "bilibili_app_splash")]
        pub mod splash {
            #[cfg(feature = "bilibili_app_splash_v1")]
            pub mod v1 {
                include!("bilibili.app.splash.v1.rs");
            }
        }
        #[cfg(feature = "bilibili_app_story")]
        pub mod story {
            #[cfg(feature = "bilibili_app_story_v1")]
            pub mod v1 {
                include!("bilibili.app.story.v1.rs");
            }
        }
        #[cfg(feature = "bilibili_app_topic")]
        pub mod topic {
            #[cfg(feature = "bilibili_app_topic_v1")]
            pub mod v1 {
                include!("bilibili.app.topic.v1.rs");
            }
        }
        #[cfg(feature = "bilibili_app_view")]
        pub mod view {
            #[cfg(feature = "bilibili_app_view_v1")]
            pub mod v1 {
                include!("bilibili.app.view.v1.rs");
            }
        }
        #[cfg(feature = "bilibili_app_viewunite")]
        pub mod viewunite {
            #[cfg(feature = "bilibili_app_viewunite_common")]
            pub mod common {
                include!("bilibili.app.viewunite.common.rs");
            }
            #[cfg(feature = "bilibili_app_viewunite_pgcanymodel")]
            pub mod pgcanymodel {
                include!("bilibili.app.viewunite.pgcanymodel.rs");
            }
            #[cfg(feature = "bilibili_app_viewunite_pugvanymodel")]
            pub mod pugvanymodel {
                include!("bilibili.app.viewunite.pugvanymodel.rs");
            }
            #[cfg(feature = "bilibili_app_viewunite_ugcanymodel")]
            pub mod ugcanymodel {
                include!("bilibili.app.viewunite.ugcanymodel.rs");
            }
            #[cfg(feature = "bilibili_app_viewunite_v1")]
            pub mod v1 {
                include!("bilibili.app.viewunite.v1.rs");
            }
        }
        #[cfg(feature = "bilibili_app_wall")]
        pub mod wall {
            #[cfg(feature = "bilibili_app_wall_v1")]
            pub mod v1 {
                include!("bilibili.app.wall.v1.rs");
            }
        }
    }
    #[cfg(feature = "bilibili_broadcast")]
    pub mod broadcast {
        #[cfg(feature = "bilibili_broadcast_live")]
        pub mod live {
            include!("bilibili.broadcast.live.rs");
            #[cfg(feature = "bilibili_broadcast_live_multi_conn")]
            pub mod multi_conn {
                include!("bilibili.broadcast.live.multi_conn.rs");
            }
            #[cfg(feature = "bilibili_broadcast_live_pk")]
            pub mod pk {
                include!("bilibili.broadcast.live.pk.rs");
            }
            #[cfg(feature = "bilibili_broadcast_live_universal_interact")]
            pub mod universal_interact {
                include!("bilibili.broadcast.live.universal_interact.rs");
            }
            #[cfg(feature = "bilibili_broadcast_live_voice_room")]
            pub mod voice_room {
                include!("bilibili.broadcast.live.voice_room.rs");
            }
        }
        #[cfg(feature = "bilibili_broadcast_message")]
        pub mod message {
            #[cfg(feature = "bilibili_broadcast_message_archive")]
            pub mod archive {
                include!("bilibili.broadcast.message.archive.rs");
            }
            #[cfg(feature = "bilibili_broadcast_message_bgroup")]
            pub mod bgroup {
                include!("bilibili.broadcast.message.bgroup.rs");
            }
            #[cfg(feature = "bilibili_broadcast_message_gamecenter")]
            pub mod gamecenter {
                include!("bilibili.broadcast.message.gamecenter.rs");
            }
            #[cfg(feature = "bilibili_broadcast_message_im")]
            pub mod im {
                include!("bilibili.broadcast.message.im.rs");
            }
            #[cfg(feature = "bilibili_broadcast_message_main")]
            pub mod main {
                include!("bilibili.broadcast.message.main.rs");
            }
            #[cfg(feature = "bilibili_broadcast_message_mall")]
            pub mod mall {
                include!("bilibili.broadcast.message.mall.rs");
            }
            #[cfg(feature = "bilibili_broadcast_message_push")]
            pub mod push {
                include!("bilibili.broadcast.message.push.rs");
            }
            #[cfg(feature = "bilibili_broadcast_message_reply")]
            pub mod reply {
                include!("bilibili.broadcast.message.reply.rs");
            }
            #[cfg(feature = "bilibili_broadcast_message_topic")]
            pub mod topic {
                include!("bilibili.broadcast.message.topic.rs");
            }
            #[cfg(feature = "bilibili_broadcast_message_tv")]
            pub mod tv {
                include!("bilibili.broadcast.message.tv.rs");
            }
        }
        #[cfg(feature = "bilibili_broadcast_v1")]
        pub mod v1 {
            include!("bilibili.broadcast.v1.rs");
        }
        #[cfg(feature = "bilibili_broadcast_v2")]
        pub mod v2 {
            include!("bilibili.broadcast.v2.rs");
        }
    }
    #[cfg(feature = "bilibili_cheese")]
    pub mod cheese {
        #[cfg(feature = "bilibili_cheese_gateway")]
        pub mod gateway {
            #[cfg(feature = "bilibili_cheese_gateway_player")]
            pub mod player {
                #[cfg(feature = "bilibili_cheese_gateway_player_v1")]
                pub mod v1 {
                    include!("bilibili.cheese.gateway.player.v1.rs");
                }
            }
        }
    }
    #[cfg(feature = "bilibili_community")]
    pub mod community {
        #[cfg(feature = "bilibili_community_interfacess")]
        pub mod interfacess {
            #[cfg(feature = "bilibili_community_interfacess_biligram")]
            pub mod biligram {
                #[cfg(feature = "bilibili_community_interfacess_biligram_v1")]
                pub mod v1 {
                    include!("bilibili.community.interfacess.biligram.v1.rs");
                }
            }
            #[cfg(feature = "bilibili_community_interfacess_cosmoconn")]
            pub mod cosmoconn {
                #[cfg(feature = "bilibili_community_interfacess_cosmoconn_v1")]
                pub mod v1 {
                    include!("bilibili.community.interfacess.cosmoconn.v1.rs");
                }
            }
        }
        #[cfg(feature = "bilibili_community_service")]
        pub mod service {
            #[cfg(feature = "bilibili_community_service_cert")]
            pub mod cert {
                #[cfg(feature = "bilibili_community_service_cert_v1")]
                pub mod v1 {
                    include!("bilibili.community.service.cert.v1.rs");
                }
            }
            #[cfg(feature = "bilibili_community_service_dm")]
            pub mod dm {
                #[cfg(feature = "bilibili_community_service_dm_v1")]
                pub mod v1 {
                    include!("bilibili.community.service.dm.v1.rs");
                }
            }
            #[cfg(feature = "bilibili_community_service_govern")]
            pub mod govern {
                #[cfg(feature = "bilibili_community_service_govern_v1")]
                pub mod v1 {
                    include!("bilibili.community.service.govern.v1.rs");
                }
            }
        }
    }
    #[cfg(feature = "bilibili_creative_tool")]
    pub mod creative_tool {
        #[cfg(feature = "bilibili_creative_tool_editor")]
        pub mod editor {
            #[cfg(feature = "bilibili_creative_tool_editor_v2")]
            pub mod v2 {
                include!("bilibili.creative_tool.editor.v2.rs");
            }
        }
    }
    #[cfg(feature = "bilibili_dagw")]
    pub mod dagw {
        #[cfg(feature = "bilibili_dagw_component")]
        pub mod component {
            #[cfg(feature = "bilibili_dagw_component_avatar")]
            pub mod avatar {
                #[cfg(feature = "bilibili_dagw_component_avatar_common")]
                pub mod common {
                    include!("bilibili.dagw.component.avatar.common.rs");
                }
                #[cfg(feature = "bilibili_dagw_component_avatar_v1")]
                pub mod v1 {
                    include!("bilibili.dagw.component.avatar.v1.rs");
                    #[cfg(feature = "bilibili_dagw_component_avatar_v1_plugin")]
                    pub mod plugin {
                        include!("bilibili.dagw.component.avatar.v1.plugin.rs");
                    }
                }
            }
        }
    }
    #[cfg(feature = "bilibili_dynamic")]
    pub mod dynamic {
        #[cfg(feature = "bilibili_dynamic_common")]
        pub mod common {
            include!("bilibili.dynamic.common.rs");
        }
        #[cfg(feature = "bilibili_dynamic_interfaces")]
        pub mod interfaces {
            #[cfg(feature = "bilibili_dynamic_interfaces_campus")]
            pub mod campus {
                #[cfg(feature = "bilibili_dynamic_interfaces_campus_v1")]
                pub mod v1 {
                    include!("bilibili.dynamic.interfaces.campus.v1.rs");
                }
            }
            #[cfg(feature = "bilibili_dynamic_interfaces_feed")]
            pub mod feed {
                #[cfg(feature = "bilibili_dynamic_interfaces_feed_v1")]
                pub mod v1 {
                    include!("bilibili.dynamic.interfaces.feed.v1.rs");
                }
            }
            #[cfg(feature = "bilibili_dynamic_interfaces_vote")]
            pub mod vote {
                #[cfg(feature = "bilibili_dynamic_interfaces_vote_v1")]
                pub mod v1 {
                    include!("bilibili.dynamic.interfaces.vote.v1.rs");
                }
            }
        }
    }
    #[cfg(feature = "bilibili_gaia")]
    pub mod gaia {
        #[cfg(feature = "bilibili_gaia_gw")]
        pub mod gw {
            include!("bilibili.gaia.gw.rs");
        }
    }
    #[cfg(feature = "bilibili_im")]
    pub mod im {
        #[cfg(feature = "bilibili_im_customer")]
        pub mod customer {
            #[cfg(feature = "bilibili_im_customer_independent")]
            pub mod independent {
                include!("bilibili.im.customer.independent.rs");
            }
            #[cfg(feature = "bilibili_im_customer_interfaces")]
            pub mod interfaces {
                include!("bilibili.im.customer.interfaces.rs");
            }
            #[cfg(feature = "bilibili_im_customer_model")]
            pub mod model {
                include!("bilibili.im.customer.model.rs");
            }
        }
        #[cfg(feature = "bilibili_im_gateway")]
        pub mod gateway {
            #[cfg(feature = "bilibili_im_gateway_interfaces")]
            pub mod interfaces {
                #[cfg(feature = "bilibili_im_gateway_interfaces_v1")]
                pub mod v1 {
                    include!("bilibili.im.gateway.interfaces.v1.rs");
                }
            }
        }
        #[cfg(feature = "bilibili_im_interfaces")]
        pub mod interfaces {
            #[cfg(feature = "bilibili_im_interfaces_v1")]
            pub mod v1 {
                include!("bilibili.im.interfaces.v1.rs");
            }
        }
        #[cfg(feature = "bilibili_im_type")]
        pub mod r#type {
            include!("bilibili.im.r#type.rs");
        }
    }
    #[cfg(feature = "bilibili_live")]
    pub mod live {
        #[cfg(feature = "bilibili_live_app")]
        pub mod app {
            #[cfg(feature = "bilibili_live_app_interfaces")]
            pub mod interfaces {
                #[cfg(feature = "bilibili_live_app_interfaces_api")]
                pub mod api {
                    #[cfg(feature = "bilibili_live_app_interfaces_api_grpc")]
                    pub mod grpc {
                        #[cfg(feature = "bilibili_live_app_interfaces_api_grpc_v1")]
                        pub mod v1 {
                            include!("bilibili.live.app.interfaces.api.grpc.v1.rs");
                        }
                    }
                }
            }
            #[cfg(feature = "bilibili_live_app_room")]
            pub mod room {
                #[cfg(feature = "bilibili_live_app_room_v1")]
                pub mod v1 {
                    include!("bilibili.live.app.room.v1.rs");
                }
            }
        }
        #[cfg(feature = "bilibili_live_approom")]
        pub mod approom {
            #[cfg(feature = "bilibili_live_approom_api")]
            pub mod api {
                #[cfg(feature = "bilibili_live_approom_api_grpc")]
                pub mod grpc {
                    #[cfg(feature = "bilibili_live_approom_api_grpc_v1")]
                    pub mod v1 {
                        include!("bilibili.live.approom.api.grpc.v1.rs");
                    }
                }
            }
        }
        #[cfg(feature = "bilibili_live_play_gateway")]
        pub mod play_gateway {
            include!("bilibili.live.play_gateway.rs");
        }
        #[cfg(feature = "bilibili_live_rtc")]
        pub mod rtc {
            include!("bilibili.live.rtc.rs");
            #[cfg(feature = "bilibili_live_rtc_datachannel")]
            pub mod datachannel {
                include!("bilibili.live.rtc.datachannel.rs");
                #[cfg(feature = "bilibili_live_rtc_datachannel_report")]
                pub mod report {
                    include!("bilibili.live.rtc.datachannel.report.rs");
                }
            }
        }
        #[cfg(feature = "bilibili_live_xroom_extend")]
        pub mod xroom_extend {
            #[cfg(feature = "bilibili_live_xroom_extend_api")]
            pub mod api {
                #[cfg(feature = "bilibili_live_xroom_extend_api_v1")]
                pub mod v1 {
                    include!("bilibili.live.xroom_extend.api.v1.rs");
                }
            }
        }
    }
    #[cfg(feature = "bilibili_main")]
    pub mod main {
        #[cfg(feature = "bilibili_main_community")]
        pub mod community {
            #[cfg(feature = "bilibili_main_community_reply")]
            pub mod reply {
                #[cfg(feature = "bilibili_main_community_reply_v1")]
                pub mod v1 {
                    include!("bilibili.main.community.reply.v1.rs");
                }
                #[cfg(feature = "bilibili_main_community_reply_v2")]
                pub mod v2 {
                    include!("bilibili.main.community.reply.v2.rs");
                }
            }
        }
    }
    #[cfg(feature = "bilibili_mall")]
    pub mod mall {
        #[cfg(feature = "bilibili_mall_tab3")]
        pub mod tab3 {
            #[cfg(feature = "bilibili_mall_tab3_dynamic")]
            pub mod dynamic {
                #[cfg(feature = "bilibili_mall_tab3_dynamic_v1")]
                pub mod v1 {
                    include!("bilibili.mall.tab3.dynamic.v1.rs");
                }
            }
            #[cfg(feature = "bilibili_mall_tab3_playerunite")]
            pub mod playerunite {
                #[cfg(feature = "bilibili_mall_tab3_playerunite_ugcanymodel")]
                pub mod ugcanymodel {
                    include!("bilibili.mall.tab3.playerunite.ugcanymodel.rs");
                }
                #[cfg(feature = "bilibili_mall_tab3_playerunite_v1")]
                pub mod v1 {
                    include!("bilibili.mall.tab3.playerunite.v1.rs");
                }
            }
            #[cfg(feature = "bilibili_mall_tab3_viewunite")]
            pub mod viewunite {
                #[cfg(feature = "bilibili_mall_tab3_viewunite_common")]
                pub mod common {
                    include!("bilibili.mall.tab3.viewunite.common.rs");
                }
                #[cfg(feature = "bilibili_mall_tab3_viewunite_ugcanymodel")]
                pub mod ugcanymodel {
                    include!("bilibili.mall.tab3.viewunite.ugcanymodel.rs");
                }
                #[cfg(feature = "bilibili_mall_tab3_viewunite_v1")]
                pub mod v1 {
                    include!("bilibili.mall.tab3.viewunite.v1.rs");
                }
            }
        }
    }
    #[cfg(feature = "bilibili_metadata")]
    pub mod metadata {
        include!("bilibili.metadata.rs");
        #[cfg(feature = "bilibili_metadata_device")]
        pub mod device {
            include!("bilibili.metadata.device.rs");
        }
        #[cfg(feature = "bilibili_metadata_fawkes")]
        pub mod fawkes {
            include!("bilibili.metadata.fawkes.rs");
        }
        #[cfg(feature = "bilibili_metadata_locale")]
        pub mod locale {
            include!("bilibili.metadata.locale.rs");
        }
        #[cfg(feature = "bilibili_metadata_network")]
        pub mod network {
            include!("bilibili.metadata.network.rs");
        }
        #[cfg(feature = "bilibili_metadata_parabox")]
        pub mod parabox {
            include!("bilibili.metadata.parabox.rs");
        }
        #[cfg(feature = "bilibili_metadata_restriction")]
        pub mod restriction {
            include!("bilibili.metadata.restriction.rs");
        }
    }
    #[cfg(feature = "bilibili_pagination")]
    pub mod pagination {
        include!("bilibili.pagination.rs");
    }
    #[cfg(feature = "bilibili_pangu")]
    pub mod pangu {
        #[cfg(feature = "bilibili_pangu_gallery")]
        pub mod gallery {
            #[cfg(feature = "bilibili_pangu_gallery_v1")]
            pub mod v1 {
                include!("bilibili.pangu.gallery.v1.rs");
            }
        }
    }
    #[cfg(feature = "bilibili_pgc")]
    pub mod pgc {
        #[cfg(feature = "bilibili_pgc_gateway")]
        pub mod gateway {
            #[cfg(feature = "bilibili_pgc_gateway_player")]
            pub mod player {
                #[cfg(feature = "bilibili_pgc_gateway_player_v1")]
                pub mod v1 {
                    include!("bilibili.pgc.gateway.player.v1.rs");
                }
                #[cfg(feature = "bilibili_pgc_gateway_player_v2")]
                pub mod v2 {
                    include!("bilibili.pgc.gateway.player.v2.rs");
                }
            }
        }
    }
    #[cfg(feature = "bilibili_playershared")]
    pub mod playershared {
        include!("bilibili.playershared.rs");
    }
    #[cfg(feature = "bilibili_polymer")]
    pub mod polymer {
        #[cfg(feature = "bilibili_polymer_app")]
        pub mod app {
            #[cfg(feature = "bilibili_polymer_app_search")]
            pub mod search {
                #[cfg(feature = "bilibili_polymer_app_search_v1")]
                pub mod v1 {
                    include!("bilibili.polymer.app.search.v1.rs");
                }
            }
        }
        #[cfg(feature = "bilibili_polymer_chronos")]
        pub mod chronos {
            #[cfg(feature = "bilibili_polymer_chronos_v1")]
            pub mod v1 {
                include!("bilibili.polymer.chronos.v1.rs");
            }
        }
        #[cfg(feature = "bilibili_polymer_community")]
        pub mod community {
            #[cfg(feature = "bilibili_polymer_community_govern")]
            pub mod govern {
                #[cfg(feature = "bilibili_polymer_community_govern_v1")]
                pub mod v1 {
                    include!("bilibili.polymer.community.govern.v1.rs");
                }
            }
        }
        #[cfg(feature = "bilibili_polymer_contract")]
        pub mod contract {
            include!("bilibili.polymer.contract.rs");
        }
        #[cfg(feature = "bilibili_polymer_list")]
        pub mod list {
            include!("bilibili.polymer.list.rs");
        }
    }
    #[cfg(feature = "bilibili_relation")]
    pub mod relation {
        #[cfg(feature = "bilibili_relation_interfaces")]
        pub mod interfaces {
            include!("bilibili.relation.interfaces.rs");
        }
    }
    #[cfg(feature = "bilibili_rpc")]
    pub mod rpc {
        include!("bilibili.rpc.rs");
    }
    #[cfg(feature = "bilibili_vas")]
    pub mod vas {
        #[cfg(feature = "bilibili_vas_garb")]
        pub mod garb {
            #[cfg(feature = "bilibili_vas_garb_model")]
            pub mod model {
                include!("bilibili.vas.garb.model.rs");
            }
            #[cfg(feature = "bilibili_vas_garb_service")]
            pub mod service {
                include!("bilibili.vas.garb.service.rs");
            }
        }
    }
    #[cfg(feature = "bilibili_vega")]
    pub mod vega {
        #[cfg(feature = "bilibili_vega_deneb")]
        pub mod deneb {
            #[cfg(feature = "bilibili_vega_deneb_v1")]
            pub mod v1 {
                include!("bilibili.vega.deneb.v1.rs");
            }
        }
    }
}
#[cfg(feature = "pgc")]
pub mod pgc {
    #[cfg(feature = "pgc_biz")]
    pub mod biz {
        include!("pgc.biz.rs");
    }
    #[cfg(feature = "pgc_gateway")]
    pub mod gateway {
        #[cfg(feature = "pgc_gateway_vega")]
        pub mod vega {
            #[cfg(feature = "pgc_gateway_vega_v1")]
            pub mod v1 {
                include!("pgc.gateway.vega.v1.rs");
            }
        }
    }
}
