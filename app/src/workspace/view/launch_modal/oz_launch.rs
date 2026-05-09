use super::{CTAButton, CheckboxConfig, LaunchModalEvent, Slide};
use crate::ai::ambient_agents::telemetry::{CloudAgentTelemetryEvent, CloudModeEntryPoint};
use crate::terminal::view::OnboardingIntention;
use crate::ui_components::icons::Icon;
use crate::workspace::action::WorkspaceAction;
use crate::workspace::view::OnboardingTutorial;
use crate::workspaces::user_workspaces::UserWorkspaces;
use crate::workspaces::workspace::{AdminEnablementSetting, UgcCollectionEnablementSetting};
use asset_macro::bundled_or_fetched_asset;
use markdown_parser::{FormattedTextFragment, FormattedTextLine};
use warp_core::send_telemetry_from_ctx;
use warpui::assets::asset_cache::AssetSource;
use warpui::{AppContext, SingletonEntity};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OzLaunchSlide {
    CloudAgents,
    AgentAutomations,
    AgentManagement,
    LaunchCredits,
}

impl Slide for OzLaunchSlide {
    fn modal_title(&self) -> String {
        "Oz 介绍".to_string()
    }

    fn modal_subtext_paragraphs(&self) -> Vec<FormattedTextLine> {
        vec![FormattedTextLine::Line(vec![
            FormattedTextFragment::plain_text(
                "无限可扩展的编码代理 — 在本地或云中运行。",
            ),
        ])]
    }

    fn first() -> Self {
        OzLaunchSlide::CloudAgents
    }

    fn next(&self) -> Option<Self> {
        match self {
            OzLaunchSlide::CloudAgents => Some(OzLaunchSlide::AgentAutomations),
            OzLaunchSlide::AgentAutomations => Some(OzLaunchSlide::AgentManagement),
            OzLaunchSlide::AgentManagement => Some(OzLaunchSlide::LaunchCredits),
            OzLaunchSlide::LaunchCredits => None,
        }
    }

    fn prev(&self) -> Option<Self> {
        match self {
            OzLaunchSlide::CloudAgents => None,
            OzLaunchSlide::AgentAutomations => Some(OzLaunchSlide::CloudAgents),
            OzLaunchSlide::AgentManagement => Some(OzLaunchSlide::AgentAutomations),
            OzLaunchSlide::LaunchCredits => Some(OzLaunchSlide::AgentManagement),
        }
    }

    fn display_text(&self) -> Option<&'static str> {
        Some(match self {
            OzLaunchSlide::CloudAgents => "云端代理",
            OzLaunchSlide::AgentAutomations => "代理自动化",
            OzLaunchSlide::AgentManagement => "代理管理",
            OzLaunchSlide::LaunchCredits => "一点小礼物",
        })
    }

    fn short_label(&self) -> &'static str {
        match self {
            OzLaunchSlide::CloudAgents => "云端代理",
            OzLaunchSlide::AgentAutomations => "代理自动化",
            OzLaunchSlide::AgentManagement => "代理管理",
            OzLaunchSlide::LaunchCredits => "启动积分",
        }
    }

    fn title(&self) -> &'static str {
        match self {
            OzLaunchSlide::CloudAgents => "使用云端代理突破笔记本电脑的限制",
            OzLaunchSlide::AgentAutomations => {
                "编排代理，将技能转化为自动化"
            }
            OzLaunchSlide::AgentManagement => "无缝追踪本地和云端代理",
            OzLaunchSlide::LaunchCredits => {
                "升级到 Warp Build 即可获得 1,000 个免费云端代理积分"
            }
        }
    }

    fn title_icon(&self) -> Option<Icon> {
        None
    }

    fn content(&self) -> &'static str {
        match self {
            OzLaunchSlide::CloudAgents => {
                "使用云端代理并行运行多个代理，在关闭笔记本电脑后让代理继续工作，或通过编程方式启动代理。此外，您还可以通过网络查看它们的工作进度。"
            }
            OzLaunchSlide::AgentAutomations => {
                "Oz 代理可以使用标准的技能格式进行定义。您可以使用内置调度器设置代理按固定间隔自主运行，或使用 Oz SDK 或 API 以编程方式启动和管理 Oz 代理。"
            }
            OzLaunchSlide::AgentManagement => {
                "在 Warp 应用或 [oz.warp.dev](https://oz.warp.dev) 上查看您所有本地和云端的代理。加入实时代理会话，在本地继续任务，一键操控代理。"
            }
            OzLaunchSlide::LaunchCredits => {
                "本月升级到 Build 套餐即可获得 1,000 额外积分用于体验 Oz。积分仅适用于在 Warp 托管的云环境中运行 Oz。"
            }
        }
    }

    fn image(&self) -> AssetSource {
        // TODO: Replace with new images once provided.
        match self {
            OzLaunchSlide::CloudAgents => {
                bundled_or_fetched_asset!("png/oz_cloud_agents.png")
            }
            OzLaunchSlide::AgentAutomations => {
                bundled_or_fetched_asset!("png/oz_agent_automations.png")
            }
            OzLaunchSlide::AgentManagement => {
                bundled_or_fetched_asset!("png/oz_agent_management.png")
            }
            OzLaunchSlide::LaunchCredits => {
                bundled_or_fetched_asset!("png/oz_launch_credits.png")
            }
        }
    }

    fn all() -> Vec<Self> {
        vec![
            OzLaunchSlide::CloudAgents,
            OzLaunchSlide::AgentAutomations,
            OzLaunchSlide::AgentManagement,
            OzLaunchSlide::LaunchCredits,
        ]
    }

    fn cta_button(&self) -> CTAButton<Self> {
        match self {
            OzLaunchSlide::CloudAgents
            | OzLaunchSlide::AgentAutomations
            | OzLaunchSlide::AgentManagement => {
                let next = self.next().expect("Non-final slides should have a next");
                CTAButton::next_slide(next, format!("下一步：{}", next.short_label()))
            }
            OzLaunchSlide::LaunchCredits => CTAButton::custom("立即体验", |ctx| {
                send_telemetry_from_ctx!(
                    CloudAgentTelemetryEvent::EnteredCloudMode {
                        entry_point: CloudModeEntryPoint::OzLaunchModal,
                    },
                    ctx
                );
                ctx.emit(LaunchModalEvent::Close);
                ctx.dispatch_typed_action(&WorkspaceAction::StartAgentOnboardingTutorial(
                    OnboardingTutorial::NoProject {
                        intention: OnboardingIntention::AgentDrivenDevelopment,
                    },
                ));
                ctx.dispatch_typed_action(&WorkspaceAction::AddAmbientAgentTab);
            }),
        }
    }

    fn secondary_cta_button(&self) -> Option<CTAButton<Self>> {
        match self {
            OzLaunchSlide::LaunchCredits => Some(CTAButton::close("暂时跳过")),
            OzLaunchSlide::CloudAgents
            | OzLaunchSlide::AgentAutomations
            | OzLaunchSlide::AgentManagement => None,
        }
    }

    fn checkbox_config(&self) -> Option<CheckboxConfig> {
        Some(CheckboxConfig {
            label: "将对话同步到云端",
            description: "存储在云端的代理对话可以一键与任何人共享，并支持跨设备和退出登录后继续对话。",
        })
    }

    fn should_show_checkbox(&self, app: &AppContext) -> bool {
        let cloud_storage_setting =
            UserWorkspaces::as_ref(app).get_cloud_conversation_storage_enablement_setting();
        let ugc_setting = UserWorkspaces::as_ref(app).get_ugc_collection_enablement_setting();

        // Show checkbox only when user has control over cloud storage AND UGC is not force-enabled.
        matches!(
            cloud_storage_setting,
            AdminEnablementSetting::RespectUserSetting
        ) && !matches!(ugc_setting, UgcCollectionEnablementSetting::Enable)
    }

    fn on_close(&self, ctx: &mut warpui::ViewContext<super::LaunchModal<Self>>) {
        ctx.dispatch_typed_action(&WorkspaceAction::StartAgentOnboardingTutorial(
            OnboardingTutorial::NoProject {
                intention: OnboardingIntention::AgentDrivenDevelopment,
            },
        ));
    }
}

pub fn init(app: &mut warpui::AppContext) {
    super::init::<OzLaunchSlide>(app);
}
