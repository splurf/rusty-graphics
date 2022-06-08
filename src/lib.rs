mod base;

/**
 * Manual labor for now but this is nasty right here
 */
pub mod util {
    pub mod keyboard {
        pub use sdl2::keyboard::{Keycode, Mod, Scancode};
    }

    pub mod mouse {
        pub use sdl2::mouse::{MouseButton, MouseState, MouseWheelDirection};
    }

    pub mod event {
        pub use sdl2::event::{DisplayEvent, WindowEvent as RawWindowEvent};
        use {
            super::{super::error::Error, keyboard::*, mouse::*},
            sdl2::event::Event,
        };

        #[derive(Debug)]
        pub enum KeyBoardEvent {
            KeyDown {
                timestamp: u32,
                window_id: u32,
                keycode: Option<Keycode>,
                scancode: Option<Scancode>,
                keymod: Mod,
                repeat: bool,
            },
            KeyUp {
                timestamp: u32,
                window_id: u32,
                keycode: Option<Keycode>,
                scancode: Option<Scancode>,
                keymod: Mod,
                repeat: bool,
            },
        }

        #[derive(Debug)]
        pub enum MouseEvent {
            MouseMotion {
                timestamp: u32,
                window_id: u32,
                which: u32,
                mousestate: MouseState,
                x: i32,
                y: i32,
                xrel: i32,
                yrel: i32,
            },

            MouseButtonDown {
                timestamp: u32,
                window_id: u32,
                which: u32,
                mouse_btn: MouseButton,
                clicks: u8,
                x: i32,
                y: i32,
            },
            MouseButtonUp {
                timestamp: u32,
                window_id: u32,
                which: u32,
                mouse_btn: MouseButton,
                clicks: u8,
                x: i32,
                y: i32,
            },

            MouseWheel {
                timestamp: u32,
                window_id: u32,
                which: u32,
                x: i32,
                y: i32,
                direction: MouseWheelDirection,
            },
        }

        #[derive(Debug)]
        pub enum WindowEvent {
            Quit {
                timestamp: u32,
            },
            AppTerminating {
                timestamp: u32,
            },
            AppLowMemory {
                timestamp: u32,
            },
            AppWillEnterBackground {
                timestamp: u32,
            },
            AppDidEnterBackground {
                timestamp: u32,
            },
            AppWillEnterForeground {
                timestamp: u32,
            },
            AppDidEnterForeground {
                timestamp: u32,
            },

            Display {
                timestamp: u32,
                display_index: i32,
                display_event: DisplayEvent,
            },
            Window {
                timestamp: u32,
                window_id: u32,
                win_event: RawWindowEvent,
            },
        }

        #[derive(Debug)]
        pub enum EventType {
            KeyBoard(KeyBoardEvent),
            Mouse(MouseEvent),
            Window(WindowEvent),
        }

        impl TryFrom<Event> for EventType {
            type Error = Error;

            fn try_from(event: Event) -> Result<Self, Self::Error> {
                match event {
                    _ if event.is_keyboard() => Ok(match event {
                        Event::KeyDown {
                            timestamp,
                            window_id,
                            keycode,
                            scancode,
                            keymod,
                            repeat,
                        } => Self::KeyBoard(KeyBoardEvent::KeyDown {
                            timestamp,
                            window_id,
                            keycode,
                            scancode,
                            keymod,
                            repeat,
                        }),
                        Event::KeyUp {
                            timestamp,
                            window_id,
                            keycode,
                            scancode,
                            keymod,
                            repeat,
                        } => Self::KeyBoard(KeyBoardEvent::KeyUp {
                            timestamp,
                            window_id,
                            keycode,
                            scancode,
                            keymod,
                            repeat,
                        }),
                        _ => unreachable!(),
                    }),
                    _ if event.is_mouse() => Ok(match event {
                        Event::MouseButtonDown {
                            timestamp,
                            window_id,
                            which,
                            mouse_btn,
                            clicks,
                            x,
                            y,
                        } => Self::Mouse(MouseEvent::MouseButtonDown {
                            timestamp,
                            window_id,
                            which,
                            mouse_btn,
                            clicks,
                            x,
                            y,
                        }),
                        Event::MouseButtonUp {
                            timestamp,
                            window_id,
                            which,
                            mouse_btn,
                            clicks,
                            x,
                            y,
                        } => Self::Mouse(MouseEvent::MouseButtonUp {
                            timestamp,
                            window_id,
                            which,
                            mouse_btn,
                            clicks,
                            x,
                            y,
                        }),
                        Event::MouseMotion {
                            timestamp,
                            window_id,
                            which,
                            mousestate,
                            x,
                            y,
                            xrel,
                            yrel,
                        } => Self::Mouse(MouseEvent::MouseMotion {
                            timestamp,
                            window_id,
                            which,
                            mousestate,
                            x,
                            y,
                            xrel,
                            yrel,
                        }),
                        Event::MouseWheel {
                            timestamp,
                            window_id,
                            which,
                            x,
                            y,
                            direction,
                        } => Self::Mouse(MouseEvent::MouseWheel {
                            timestamp,
                            window_id,
                            which,
                            x,
                            y,
                            direction,
                        }),
                        _ => unreachable!(),
                    }),
                    _ if event.is_window() => Ok(match event {
                        Event::Quit { timestamp } => Self::Window(WindowEvent::Quit { timestamp }),
                        Event::AppTerminating { timestamp } => {
                            Self::Window(WindowEvent::AppTerminating { timestamp })
                        }
                        Event::AppLowMemory { timestamp } => {
                            Self::Window(WindowEvent::AppLowMemory { timestamp })
                        }
                        Event::AppWillEnterBackground { timestamp } => {
                            Self::Window(WindowEvent::AppWillEnterBackground { timestamp })
                        }
                        Event::AppDidEnterBackground { timestamp } => {
                            Self::Window(WindowEvent::AppDidEnterBackground { timestamp })
                        }
                        Event::AppWillEnterForeground { timestamp } => {
                            Self::Window(WindowEvent::AppWillEnterForeground { timestamp })
                        }
                        Event::AppDidEnterForeground { timestamp } => {
                            Self::Window(WindowEvent::AppDidEnterForeground { timestamp })
                        }
                        Event::Window {
                            timestamp,
                            window_id,
                            win_event,
                        } => Self::Window(WindowEvent::Window {
                            timestamp,
                            window_id,
                            win_event,
                        }),
                        _ => unreachable!(),
                    }),
                    _ => Err(Error::from("Invalid event".to_string())),
                }
            }
        }
    }
}

pub use base::*;
