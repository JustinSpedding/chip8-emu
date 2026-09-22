use crate::cpu;
use crate::init;
use crate::state::State;
use iced::widget::canvas::{self, Canvas};
use iced::widget::{Text, button, column, row};
use iced::{
    Alignment, Color, Element, Event, Length, Point, Rectangle, Size, Subscription, Task, Theme,
    event::{Status, listen_with},
    keyboard, mouse, time, window,
};
use iced_aw::number_input;
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

#[derive(Debug, Clone, Copy)]
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

impl Chip8Emu {
    fn new(flags: Chip8EmuFlags) -> Self {
        Self {
            state: None,
            cycles_per_tick: flags.cycles_per_tick,
            ticks_per_second: flags.ticks_per_second,
            paused: true,
            canvas: Chip8EmuCanvas::default(),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        let game_tick = if self.paused {
            Subscription::none()
        } else {
            time::every(Duration::from_secs_f64(1.0 / f64::from(self.ticks_per_second))).map(|_| Message::GameTick)
        };

        Subscription::batch([game_tick, listen_with(handle_event)])
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::GameTick => {
                if let Some(state) = &mut self.state && !self.paused
                {
                    cpu::run_cycle(state, self.cycles_per_tick);
                    if state.video != self.canvas.video {
                        self.canvas.video = state.video;
                        self.canvas.canvas_cache.clear();
                    }
                }
            }
            Message::TogglePause => {
                self.paused = !self.paused;
            }
            Message::KeyDown(key_num) => {
                if let Some(state) = &mut self.state {
                    state.keypad[key_num as usize] = true;
                }
            }
            Message::KeyUp(key_num) => {
                if let Some(state) = &mut self.state {
                    state.keypad[key_num as usize] = false;
                }
            }
            Message::LoadRom => {
                let rom_path = FileDialog::new().add_filter("CHIP-8 ROM", &["ch8", "CH8"]).pick_file();
                if let Some(rom_path) = rom_path {
                    self.state = Some(init::init_state(rom_path.as_path()));
                    self.paused = false;
                }
            }
            Message::SetCyclesPerTick(cycles_per_tick) => {
                self.cycles_per_tick = cycles_per_tick;
            }
            Message::SetTicksPerSecond(ticks_per_second) => {
                self.ticks_per_second = ticks_per_second;
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let controls = row![
            button("Load Rom").padding([5, 10]).on_press(Message::LoadRom),
            Text::new("Cycles per tick:"),
            number_input(&self.cycles_per_tick, 1..255, Message::SetCyclesPerTick)
                .style(number_input::number_input::primary)
                .width(Length::Fixed(60.0))
                .step(1),
            Text::new("Ticks per second:"),
            number_input(&self.ticks_per_second, 1..250, Message::SetTicksPerSecond)
                .style(number_input::number_input::primary)
                .width(Length::Fixed(60.0))
                .step(10),
        ]
        .spacing(10)
        .padding(10)
        .align_y(Alignment::Center)
        .height(Length::Shrink);

        column![controls, self.canvas.view()].width(Length::Fill).height(Length::Fill).into()
    }
}

pub fn create_ui() {
    let flags = Chip8EmuFlags::default();

    iced::application(move || Chip8Emu::new(flags), Chip8Emu::update, Chip8Emu::view)
        .title("Chip-8 Emulator")
        .subscription(Chip8Emu::subscription)
        .font(iced_aw::ICED_AW_FONT_BYTES)
        .run()
        .expect("Failed to launch application.");
}

fn handle_event(event: Event, status: Status, _window: window::Id) -> Option<Message> {
    if status == Status::Captured {
        return None;
    }

    match event {
        Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => keypad_index(&key).map(Message::KeyDown),
        Event::Keyboard(keyboard::Event::KeyReleased { key, .. }) => match keypad_index(&key) {
            Some(key_num) => Some(Message::KeyUp(key_num)),
            None => match key {
                keyboard::Key::Named(keyboard::key::Named::Space) => Some(Message::TogglePause),
                _ => None,
            },
        },
        _ => None,
    }
}

fn keypad_index(key: &keyboard::Key) -> Option<u8> {
    let keyboard::Key::Character(character) = key.as_ref() else {
        return None;
    };

    match character.to_ascii_uppercase().as_str() {
        "0" => Some(0),
        "1" => Some(1),
        "2" => Some(2),
        "3" => Some(3),
        "Q" => Some(4),
        "W" => Some(5),
        "E" => Some(6),
        "R" => Some(7),
        "A" => Some(8),
        "S" => Some(9),
        "D" => Some(10),
        "F" => Some(11),
        "Z" => Some(12),
        "X" => Some(13),
        "C" => Some(14),
        "V" => Some(15),
        _ => None,
    }
}

#[derive(Debug, Default)]
struct Chip8EmuCanvas {
    canvas_cache: canvas::Cache,
    video: [u64; 32],
}

#[derive(Debug, Default)]
struct Chip8EmuCanvasState {}

impl Chip8EmuCanvas {
    pub fn view(&self) -> Element<'_, Message> {
        Canvas::new(self).width(Length::Fill).height(Length::Fill).into()
    }
}

impl canvas::Program<Message> for Chip8EmuCanvas {
    type State = Chip8EmuCanvasState;

    fn draw(&self, _state: &Chip8EmuCanvasState, renderer: &iced::Renderer, _theme: &Theme, bounds: Rectangle, _cursor: mouse::Cursor) -> Vec<canvas::Geometry> {
        let screen = self.canvas_cache.draw(renderer, bounds.size(), |frame| {
            let screen_size = frame.size();
            let point_size = Size {
                width: screen_size.width / 64.,
                height: screen_size.height / 32.,
            };

            // Draw a black background
            let background = canvas::Path::rectangle(Point::ORIGIN, screen_size);
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
