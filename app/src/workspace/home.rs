//! Warp Home
//!
//! This is the landing page for new tabs if session creation isn't supported (e.g. on the web).
//! It's barebones at the moment, but may grow into a more full-featured admin experience.

use warpui::ViewContext;

use super::view::Workspace;
use crate::pane_group::{AnyPaneContent, FilePane};

const WARP_HOME_TITLE: &str = "欢迎使用 Warp Web 版";
const WARP_HOME_CONTENT: &str = r#"
欢迎使用 Warp Web 版 — 您的浏览器版 Warp 大本营！
使用 Warp Web 版可以：
* 加入共享会话
* 创建、查看和编辑 Warp Drive 对象
* 管理您的 Warp 设置

尚未下载 Warp 的团队成员和同事也可以使用 Warp Web 版查看您的共享会话、笔记本和工作流。"#;

/// Create a static "home page" pane.
pub fn create_home_pane(ctx: &mut ViewContext<Workspace>) -> Box<dyn AnyPaneContent> {
    let pane = FilePane::new(
        None,
        None,
        #[cfg(feature = "local_fs")]
        None,
        ctx,
    );
    pane.file_view(ctx).update(ctx, |pane, ctx| {
        pane.open_static(WARP_HOME_TITLE, WARP_HOME_CONTENT, ctx);
    });
    Box::new(pane)
}
