use druid::{
    widget::{Button, Label},
    AppLauncher, PlatformError, Selector, Widget, WindowDesc,
};
use druid_widget_nursery::{enum_switcher::Switcher, prism::Prism, WidgetExt as _};

fn main() -> Result<(), PlatformError> {
    AppLauncher::with_window(WindowDesc::new(ui())).launch(AppState::A)
}

#[derive(Clone, PartialEq, Eq, druid::Data)]
enum AppState {
    A,
    B,
}

pub const SWITCH: Selector<()> = Selector::new("switch");

pub(crate) fn ui() -> impl Widget<AppState> {
    Switcher::new()
        .with_variant(AppStateA, state_a())
        .with_variant(AppStateB, state_b())
        .on_command(SWITCH, |_ctx, _user_data, state| {
            *state = AppState::B;
            //ctx.set_handled();
        })
}

fn state_a() -> impl Widget<()> {
    Button::new("Switch").on_click(|ctx, _state, _env| {
        ctx.submit_command(SWITCH);
    })
}

fn state_b() -> impl Widget<()> {
    Label::new("switched")
}

struct AppStateA;
struct AppStateB;

impl Prism<AppState, ()> for AppStateA {
    fn get(&self, data: &AppState) -> Option<()> {
        match data {
            AppState::A => Some(()),
            _ => None,
        }
    }

    fn put(&self, data: &mut AppState, _inner: ()) {
        *data = AppState::A;
    }
}

impl Prism<AppState, ()> for AppStateB {
    fn get(&self, data: &AppState) -> Option<()> {
        match data {
            AppState::B => Some(()),
            _ => None,
        }
    }

    fn put(&self, data: &mut AppState, _inner: ()) {
        *data = AppState::B;
    }
}
