use gpui::{
    Action, AnyElement, Hsla, MAX_BUTTONS_PER_SIDE, MouseButton, WindowButton, prelude::*, rgb, svg,
};
use ui::prelude::*;

#[derive(IntoElement)]
pub struct LinuxWindowControls {
    id: &'static str,
    buttons: [Option<WindowButton>; MAX_BUTTONS_PER_SIDE],
    close_action: Box<dyn Action>,
}

impl LinuxWindowControls {
    pub fn new(
        id: &'static str,
        buttons: [Option<WindowButton>; MAX_BUTTONS_PER_SIDE],
        close_action: Box<dyn Action>,
    ) -> Self {
        Self {
            id,
            buttons,
            close_action,
        }
    }
}

impl RenderOnce for LinuxWindowControls {
    fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let is_maximized = window.is_maximized();
        let supported_controls = window.window_controls();
        let button_elements: Vec<AnyElement> = self
            .buttons
            .iter()
            .filter_map(|b| *b)
            .filter(|button| match button {
                WindowButton::Minimize => supported_controls.minimize,
                WindowButton::Maximize => supported_controls.maximize,
                WindowButton::Close => true,
            })
            .map(|button| {
                create_window_button(button, button.id(), is_maximized, &*self.close_action)
            })
            .collect();

        h_flex()
            .id(self.id)
            .when(!button_elements.is_empty(), |el| {
                el.gap(px(8.0))
                    .px_3()
                    .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                    .children(button_elements)
            })
    }
}

fn create_window_button(
    button: WindowButton,
    id: &'static str,
    is_maximized: bool,
    close_action: &dyn Action,
) -> AnyElement {
    match button {
        WindowButton::Minimize => {
            WindowControl::new(id, WindowControlType::Minimize).into_any_element()
        }
        WindowButton::Maximize => WindowControl::new(
            id,
            if is_maximized {
                WindowControlType::Restore
            } else {
                WindowControlType::Maximize
            },
        )
        .into_any_element(),
        WindowButton::Close => {
            WindowControl::new_close(id, WindowControlType::Close, close_action.boxed_clone())
                .into_any_element()
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum WindowControlType {
    Minimize,
    Restore,
    Maximize,
    Close,
}

impl WindowControlType {
    /// Returns the icon name for the window control type.
    pub fn icon(&self) -> IconName {
        match self {
            WindowControlType::Minimize => IconName::GenericMinimize,
            WindowControlType::Restore => IconName::GenericRestore,
            WindowControlType::Maximize => IconName::GenericMaximize,
            WindowControlType::Close => IconName::GenericClose,
        }
    }

    fn background(self) -> Hsla {
        rgb(match self {
            Self::Close => 0xe9524a,
            Self::Minimize => 0xf1ae1b,
            Self::Maximize | Self::Restore => 0x59c837,
        })
        .into()
    }
}

#[derive(IntoElement)]
pub struct WindowControl {
    id: ElementId,
    icon: WindowControlType,
    close_action: Option<Box<dyn Action>>,
}

impl WindowControl {
    pub fn new(id: impl Into<ElementId>, icon: WindowControlType) -> Self {
        Self {
            id: id.into(),
            icon,
            close_action: None,
        }
    }

    pub fn new_close(
        id: impl Into<ElementId>,
        icon: WindowControlType,
        close_action: Box<dyn Action>,
    ) -> Self {
        Self {
            id: id.into(),
            icon,
            close_action: Some(close_action.boxed_clone()),
        }
    }
}

impl RenderOnce for WindowControl {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let enabled = match self.icon {
            WindowControlType::Minimize => window.is_minimizable(),
            WindowControlType::Restore | WindowControlType::Maximize => window.is_resizable(),
            WindowControlType::Close => true,
        };
        let active = window.is_window_active();
        let background = if enabled && active {
            self.icon.background()
        } else {
            cx.theme().colors().icon_disabled
        };
        let symbol: Hsla = rgb(0x302820).into();
        let pressed_background = background.blend(gpui::black().opacity(0.18));
        let icon = svg()
            .size(px(10.0))
            .flex_none()
            .path(self.icon.icon().path())
            .text_color(gpui::transparent_black())
            .when(enabled, |this| {
                this.group_hover("", |this| this.text_color(symbol))
            });

        h_flex()
            .id(self.id)
            .group("")
            .justify_center()
            .content_center()
            .rounded_full()
            .size(px(14.0))
            .flex_none()
            .bg(background)
            .when(enabled, |this| {
                this.cursor_pointer()
                    .active(|this| this.bg(pressed_background))
            })
            .child(icon)
            .on_mouse_move(|_, _, cx| cx.stop_propagation())
            .on_click(move |_, window, cx| {
                cx.stop_propagation();
                if !enabled {
                    return;
                }
                match self.icon {
                    WindowControlType::Minimize => window.minimize_window(),
                    WindowControlType::Restore => window.zoom_window(),
                    WindowControlType::Maximize => window.zoom_window(),
                    WindowControlType::Close => window.dispatch_action(
                        self.close_action
                            .as_ref()
                            .expect("Use WindowControl::new_close() for close control.")
                            .boxed_clone(),
                        cx,
                    ),
                }
            })
    }
}
