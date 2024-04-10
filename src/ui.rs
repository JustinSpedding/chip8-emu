use crate::cpu;
use crate::init;
use crate::state::State;
use iced::keyboard;
use iced::mouse;
use iced::widget::button;
use iced::widget::canvas::Geometry;
use iced::widget::row;
use iced::widget::Canvas;
use iced::widget::{canvas, column};
use iced::Color;
use iced::Point;
use iced::Rectangle;
use iced::Renderer;
use iced::Size;
use iced::{executor, time, Application, Command, Element, Length, Settings, Theme};
use rfd::FileDialog;
use std::time::Duration;

#[derive(Debug)]
struct Chip8Emu {
    state: Option<State>,
    cycles_per_tick: u8,
    ticks_per_second: u8,
    paused: bool,
    canvas: Chip8EmuCanvas,
}

#[derive(Debug)]
struct Chip8EmuFlags {
    cycles_per_tick: u8,
    ticks_per_second: u8,
}

#[derive(Debug, Clone)]
pub enum Message {
    GameTick,
    TogglePause,
    KeyDown(u8),
    KeyUp(u8),
    LoadRom,
    SetCyclesPerTick(u8),
    SetTicksPerSecond(u8),
}

impl Default for Chip8EmuFlags {
    fn default() -> Self {
        Self {
            cycles_per_tick: 4,
            ticks_per_second: 60,
        }
    }
}

impl Application for Chip8Emu {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = Chip8EmuFlags;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                state: None,
                cycles_per_tick: flags.cycles_per_tick,
                ticks_per_second: flags.ticks_per_second,
                paused: true,
                canvas: Chip8EmuCanvas::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Chip-8 Emulator")
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::Subscription::batch(vec![
            if !self.paused {
                time::every(Duration::from_secs_f64(1. / (self.ticks_per_second as f64))).map(|_| Self::Message::GameTick)
            } else {
                iced::Subscription::none()
            },
            keyboard::on_key_press(|key, _modifiers| match key.as_ref() {
                keyboard::key::Key::Character("0") => Some(Message::KeyDown(0)),
                keyboard::key::Key::Character("1") => Some(Message::KeyDown(1)),
                keyboard::key::Key::Character("2") => Some(Message::KeyDown(2)),
                keyboard::key::Key::Character("3") => Some(Message::KeyDown(3)),
                keyboard::key::Key::Character("Q") => Some(Message::KeyDown(4)),
                keyboard::key::Key::Character("W") => Some(Message::KeyDown(5)),
                keyboard::key::Key::Character("E") => Some(Message::KeyDown(6)),
                keyboard::key::Key::Character("R") => Some(Message::KeyDown(7)),
                keyboard::key::Key::Character("A") => Some(Message::KeyDown(8)),
                keyboard::key::Key::Character("S") => Some(Message::KeyDown(9)),
                keyboard::key::Key::Character("D") => Some(Message::KeyDown(10)),
                keyboard::key::Key::Character("F") => Some(Message::KeyDown(11)),
                keyboard::key::Key::Character("Z") => Some(Message::KeyDown(12)),
                keyboard::key::Key::Character("X") => Some(Message::KeyDown(13)),
                keyboard::key::Key::Character("C") => Some(Message::KeyDown(14)),
                keyboard::key::Key::Character("V") => Some(Message::KeyDown(15)),
                _ => None,
            }),
            keyboard::on_key_release(|key, _modifiers| match key.as_ref() {
                keyboard::key::Key::Character("0") => Some(Message::KeyUp(0)),
                keyboard::key::Key::Character("1") => Some(Message::KeyUp(1)),
                keyboard::key::Key::Character("2") => Some(Message::KeyUp(2)),
                keyboard::key::Key::Character("3") => Some(Message::KeyUp(3)),
                keyboard::key::Key::Character("Q") => Some(Message::KeyUp(4)),
                keyboard::key::Key::Character("W") => Some(Message::KeyUp(5)),
                keyboard::key::Key::Character("E") => Some(Message::KeyUp(6)),
                keyboard::key::Key::Character("R") => Some(Message::KeyUp(7)),
                keyboard::key::Key::Character("A") => Some(Message::KeyUp(8)),
                keyboard::key::Key::Character("S") => Some(Message::KeyUp(9)),
                keyboard::key::Key::Character("D") => Some(Message::KeyUp(10)),
                keyboard::key::Key::Character("F") => Some(Message::KeyUp(11)),
                keyboard::key::Key::Character("Z") => Some(Message::KeyUp(12)),
                keyboard::key::Key::Character("X") => Some(Message::KeyUp(13)),
                keyboard::key::Key::Character("C") => Some(Message::KeyUp(14)),
                keyboard::key::Key::Character("V") => Some(Message::KeyUp(15)),
                keyboard::key::Key::Named(keyboard::key::Named::Space) => Some(Message::TogglePause),
                _ => None,
            }),
        ])
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::GameTick => match &mut self.state {
                Some(state) => {
                    if !self.paused {
                        cpu::run_cycle(state, self.cycles_per_tick);
                        if state.video != self.canvas.video {
                            self.canvas.video = state.video;
                            self.canvas.canvas_cache.clear();
                        }
                    }
                }
                None => {}
            },
            Message::TogglePause => {
                self.paused = !self.paused;
            }
            Message::KeyDown(key_num) => match &mut self.state {
                Some(state) => {
                    state.keypad[key_num as usize] = true;
                }
                None => {}
            },
            Message::KeyUp(key_num) => match &mut self.state {
                Some(state) => {
                    state.keypad[key_num as usize] = false;
                }
                None => {}
            },
            Message::LoadRom => {
                let rom_path = FileDialog::new()
                    .add_filter("CHIP-8 ROM", &["ch8", "CH8"])
                    .pick_file();
                match rom_path {
                    Some(rom_path) => {
                        self.state = Some(init::init_state(rom_path.as_path()));
                        self.paused = false;
                    }
                    None => {}
                }
            }
            Message::SetCyclesPerTick(cycles_per_tick) => {
                self.cycles_per_tick = cycles_per_tick;
            }
            Message::SetTicksPerSecond(ticks_per_second) => {
                self.ticks_per_second = ticks_per_second;
            }
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        column![
            row([button("Load Rom").padding([5, 10]).on_press(Message::LoadRom).into()]),
            row([self.canvas.view()]),
        ].into()
    }
}

pub fn create_ui() {
    Chip8Emu::run(Settings::default()).expect("Failed to launch application.");
}

#[derive(Debug, Default)]
struct Chip8EmuCanvas {
    canvas_cache: canvas::Cache,
    video: [u64; 32],
}

#[derive(Debug, Default)]
struct Chip8EmuCanvasState {}

impl Chip8EmuCanvas {
    pub fn view(&self) -> Element<Message> {
        Canvas::new(self).width(Length::Fill).height(Length::Fill).into()
    }
}

impl canvas::Program<Message> for Chip8EmuCanvas {
    type State = Chip8EmuCanvasState;

    fn draw(&self, _state: &Chip8EmuCanvasState, renderer: &Renderer, _theme: &Theme, bounds: Rectangle, _cursor: mouse::Cursor) -> Vec<Geometry> {
        let screen = self.canvas_cache.draw(renderer, bounds.size(), |frame| {
            let screen_size = frame.size();
            let point_size = Size {
                width: screen_size.width / 64.,
                height: screen_size.height / 32.,
            };

            // Draw a black background
            let background = iced::widget::canvas::Path::rectangle(Point::ORIGIN, screen_size);
            frame.fill(&background, Color::BLACK);

            // Draw each of the white pixels
            frame.with_save(|frame| {
                for row in 0..32 {
                    let video_row = self.video[row];
                    for col in 0..64 {
                        let bitmask = 1u64.rotate_right(col + 1);
                        if video_row & bitmask != 0 {
                            let point = Point {
                                x: point_size.width * col as f32,
                                y: point_size.height * row as f32,
                            };
                            frame.fill_rectangle(point, point_size, Color::WHITE);
                        }
                    }
                }
            })
        });
        vec![screen]
    }
}
