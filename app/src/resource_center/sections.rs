use warp_core::{context_flag::ContextFlag, features::FeatureFlag};
use warpui::ViewContext;

use super::{
    ContentItem, ContentSectionData, FeatureItem, FeatureSection, FeatureSectionData,
    ResourceCenterMainView, Section, Tip, TipAction, TipHint,
};

pub fn sections(ctx: &mut ViewContext<ResourceCenterMainView>) -> Vec<Section> {
    let mut sections = vec![Section::Changelog()];

    if FeatureFlag::AvatarInTabBar.is_enabled() {
        return sections;
    }

    let get_started = FeatureSectionData {
        section_name: FeatureSection::GettingStarted,
        items: vec![
            FeatureItem::new(
                "创建第一个块",
                "运行命令以查看命令和输出的分组显示。",
                Tip::Hint(TipHint::CreateBlock),
                ctx,
            ),
            FeatureItem::new(
                "浏览块",
                "点击选择块，使用箭头键导航。",
                Tip::Hint(TipHint::BlockSelect),
                ctx,
            ),
            FeatureItem::new(
                "对块执行操作",
                "右键点击块以复制/粘贴、共享等。",
                Tip::Hint(TipHint::BlockAction),
                ctx,
            ),
            FeatureItem::new(
                "打开命令面板",
                "通过键盘访问所有 Warp 功能。",
                Tip::Action(TipAction::CommandPalette),
                ctx,
            ),
            FeatureItem::new(
                "设置主题",
                "选择主题，打造个性化 Warp。",
                Tip::Action(TipAction::ThemePicker),
                ctx,
            ),
        ],
    };
    sections.push(Section::Feature(get_started));

    let maximize_warp = FeatureSectionData {
        section_name: FeatureSection::MaximizeWarp,
        items: maximize_warp_items(ctx),
    };
    sections.push(Section::Feature(maximize_warp));

    let advanced_setup = ContentSectionData {
        section_name: FeatureSection::AdvancedSetup,
        items: vec![
            ContentItem {
                title: "使用自定义提示符",
                description: "配置 Warp 以遵循你的 PS1 设置",
                url: "https://docs.warp.dev/terminal/appearance/prompt",
                button_label: "查看文档",
            },
            ContentItem {
                title: "将 Warp 与 IDE 集成",
                description: "配置 Warp 从最常用的开发工具启动",
                url: "https://docs.warp.dev/terminal/integrations-and-plugins",
                button_label: "查看文档",
            },
            ContentItem {
                title: "Warp 团队如何使用 Warp",
                description: "了解 Warp 工程团队如何使用他们最喜爱的功能",
                url: "https://www.warp.dev/blog/how-warp-uses-warp",
                button_label: "阅读文章",
            },
        ],
    };
    sections.push(Section::Content(advanced_setup));

    sections
}

fn maximize_warp_items(ctx: &mut ViewContext<ResourceCenterMainView>) -> Vec<FeatureItem> {
    let mut maximize_warp_items = vec![];

    maximize_warp_items.push(FeatureItem::new(
        "命令搜索",
        "查找并运行之前执行过的命令、工作流程等。",
        Tip::Action(TipAction::CommandSearch),
        ctx,
    ));

    maximize_warp_items.push(FeatureItem::new(
        "AI 命令搜索",
        "使用自然语言生成 shell 命令。",
        Tip::Action(TipAction::AiCommandSearch),
        ctx,
    ));

    if ContextFlag::CreateNewSession.is_enabled() {
        maximize_warp_items.push(FeatureItem::new(
            "拆分窗格",
            "将标签拆分为多个窗格，打造理想布局。",
            Tip::Action(TipAction::SplitPane),
            ctx,
        ));
    }

    if ContextFlag::LaunchConfigurations.is_enabled() {
        maximize_warp_items.push(FeatureItem::new(
            "启动配置",
            "保存当前的窗口、标签和窗格配置。",
            Tip::Action(TipAction::SaveNewLaunchConfig),
            ctx,
        ));
    }

    maximize_warp_items
}
