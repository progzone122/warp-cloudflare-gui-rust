use iced::widget::toggler::{self, Status};
use iced::{Border, Color, Theme};
use super::{
    ACCENT_COLOR,
    TOGGLER_WARP_INACTIVE,
    //
    DEFAULT_TOGGLER_WARP_BACKGROUND_RADIUS
};

pub fn toggler_warp_style(_theme: &Theme, status: Status) -> toggler::Style {
    match status {
        Status::Active { is_toggled } => {
            if is_toggled {
                toggler::Style {
                    background: ACCENT_COLOR,
                    background_border_width: DEFAULT_TOGGLER_WARP_BACKGROUND_RADIUS,
                    background_border_color: Default::default(),
                    foreground: TOGGLER_WARP_INACTIVE,
                    foreground_border_width: 10.0,
                    foreground_border_color: Default::default(),
                }
            } else {
                toggler::Style {
                    background: TOGGLER_WARP_INACTIVE,
                    background_border_width: DEFAULT_TOGGLER_WARP_BACKGROUND_RADIUS,
                    background_border_color: Default::default(),
                    foreground: ACCENT_COLOR,
                    foreground_border_width: 10.0,
                    foreground_border_color: Default::default(),
                }
            }
        },
        Status::Hovered { is_toggled } => {
            if is_toggled {
                toggler::Style {
                    background: ACCENT_COLOR,
                    background_border_width: DEFAULT_TOGGLER_WARP_BACKGROUND_RADIUS,
                    background_border_color: Default::default(),
                    foreground: TOGGLER_WARP_INACTIVE,
                    foreground_border_width: 0.0,
                    foreground_border_color: Default::default(),
                }
            } else {
                toggler::Style {
                    background: TOGGLER_WARP_INACTIVE,
                    background_border_width: DEFAULT_TOGGLER_WARP_BACKGROUND_RADIUS,
                    background_border_color: Default::default(),
                    foreground: ACCENT_COLOR,
                    foreground_border_width: 0.0,
                    foreground_border_color: Default::default(),
                }
            }
        },
        Status::Disabled => toggler::Style {
            background: TOGGLER_WARP_INACTIVE,
            background_border_width: DEFAULT_TOGGLER_WARP_BACKGROUND_RADIUS,
            background_border_color: Default::default(),
            foreground: TOGGLER_WARP_INACTIVE,
            foreground_border_width: 10.0,
            foreground_border_color: Default::default(),
        }
    }
}