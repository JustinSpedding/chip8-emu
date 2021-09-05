use crate::cpu;
use crate::init;
use crate::state::State;
use druid::commands;
use druid::menu::{Menu, MenuItem};
use druid::widget::prelude::*;
use druid::widget::Flex;
use druid::AppDelegate;
use druid::BoxConstraints;
use druid::Color;
use druid::Command;
use druid::DelegateCtx;
use druid::Event;
use druid::EventCtx;
use druid::Handled;
use druid::LayoutCtx;
use druid::LifeCycle;
use druid::LifeCycleCtx;
use druid::PaintCtx;
use druid::Point;
use druid::Rect;
use druid::Size;
use druid::Target;
use druid::TimerToken;
use druid::UpdateCtx;
use druid::{AppLauncher, Data, Env, FileDialogOptions, FileSpec, Lens, LocalizedString, MouseButton, Widget, WidgetExt, WindowDesc};
use std::time::Duration;
use std::time::Instant;

const WINDOW_TITLE: LocalizedString<AppData> = LocalizedString::new("Hello World!");

#[derive(Clone, Data, Lens)]
struct AppData {
    state: Option<State>,
    cycles_per_clock: u8,
    paused: bool,
}

struct Chip8Widget {
    timer_id: TimerToken,
    cell_size: Size,
    last_update: Instant,
}

impl Widget<AppData> for Chip8Widget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppData, _env: &Env) {
        match event {
            Event::WindowConnected => {
                ctx.request_paint();
                let deadline = Duration::from_millis(17); // TODO: make this more accurate
                self.last_update = Instant::now();
                self.timer_id = ctx.request_timer(deadline);
            }
            Event::Timer(id) => {
                if *id == self.timer_id {
                    match &mut data.state {
                        Some(state) => {
                            if !data.paused {
                                cpu::run_cycle(state, data.cycles_per_clock);
                                ctx.request_paint();
                            }
                        }
                        None => {}
                    }
                    let deadline = Duration::from_millis(17);
                    self.last_update = Instant::now();
                    self.timer_id = ctx.request_timer(deadline);
                }
            }
            Event::MouseUp(e) => {
                if e.button == MouseButton::Left {
                    data.paused = !data.paused;
                }
            }
            _ => {}
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &AppData, _env: &Env) {}

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppData, data: &AppData, _env: &Env) {
        match &data.state {
            Some(state) => match &old_data.state {
                Some(old_state) => {
                    if state.video != old_state.video {
                        ctx.request_paint();
                    }
                }
                None => {
                    ctx.request_paint();
                }
            },
            None => {}
        }
    }

    fn layout(&mut self, _layout_ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &AppData, _env: &Env) -> Size {
        let max_size = bc.max();
        if max_size.width < max_size.height * 2. {
            return Size { width: max_size.width, height: max_size.width / 2. }
        } else {
            return Size { width: max_size.height * 2., height: max_size.height }
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppData, _env: &Env) {
        match &data.state {
            Some(state) => {
                let size: Size = ctx.size();
                let w0 = size.width / 64 as f64;
                let h0 = size.height / 32 as f64;
                let cell_size = Size { width: w0, height: h0 };
                self.cell_size = cell_size;
                for row in 0..32 {
                    let video_row = state.video[row];
                    for col in 0..64 {
                        let bitmask = 1u64.rotate_right(col + 1);
                        if video_row & bitmask != 0 {
                            let point = Point {
                                x: w0 * col as f64,
                                y: h0 * row as f64,
                            };
                            let rect = Rect::from_origin_size(point, cell_size);

                            // We devide by 2 so that the colour changes every 2 positions instead of every 1
                            ctx.fill(rect, &Color::WHITE);
                        }
                    }
                }
            }
            None => {}
        }
    }
}
pub fn create_ui() {
    // describe the main window
    let main_window = WindowDesc::new(
        Flex::column()
            .with_flex_child(
                Chip8Widget {
                    timer_id: TimerToken::INVALID,
                    cell_size: Size { width: 0.0, height: 0.0 },
                    last_update: Instant::now(),
                },
                1.0,
            )
            .background(Color::BLACK),
    )
    .menu(|_, _, _| menu())
    .title(WINDOW_TITLE)
    .window_size((400.0, 400.0));

    // create the initial app state
    let initial_state = AppData {
        state: None,
        cycles_per_clock: 4,
        paused: false,
    };

    // start the application
    AppLauncher::with_window(main_window).delegate(Delegate).launch(initial_state).expect("Failed to launch application");
}

fn menu() -> Menu<AppData> {
    let chip8 = FileSpec::new("CHIP-8 ROM", &["ch8"]);
    let open_dialog_options = FileDialogOptions::new()
        .allowed_types(vec![chip8])
        .default_type(chip8)
        .name_label("CHIP-8 ROM")
        .title("Choose a CHIP-8 ROM to load")
        .button_text("Load");
    return Menu::empty().entry(Menu::<AppData>::new(LocalizedString::new("File")).entry(
        MenuItem::<AppData>::new(LocalizedString::new("Open ROM...")).on_activate(move |ctx, _data, _env| ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(open_dialog_options.clone()))),
    ));
}

struct Delegate;

impl AppDelegate<AppData> for Delegate {
    fn command(&mut self, _ctx: &mut DelegateCtx, _target: Target, cmd: &Command, data: &mut AppData, _env: &Env) -> Handled {
        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            data.state = Some(init::init_state(file_info.path()));
            return Handled::Yes;
        }
        Handled::No
    }
}
