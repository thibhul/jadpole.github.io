macro_rules! struct_events {
    [
        keyboard: { $( $k_alias:ident : $k_sdl:ident ),* },
        window: { $( $win_alias:ident : $win_sdl:pat ),* },
        else: { $( $e_alias:ident : $e_sdl:pat ),* }
    ]
    => {
        use sdl2::EventPump;


        /// The events that occured since the last frame and will not be recorded on the
        /// next `Events::pump`. This allows, for instance, to check whether the user
        /// just clicked on a link, without having to detect changes in the mouse's
        /// state manually.
        pub struct ImmediateEvents {
            $( pub $k_alias : Option<bool> , )*
            $( pub $win_alias: bool , ),*
            $( pub $e_alias : bool ),*
        }

        impl ImmediateEvents {
            /// Return the default state of ImmediateEvents, one in which no event has
            /// yet been recorded.
            pub fn new() -> ImmediateEvents {
                ImmediateEvents {
                    $( $k_alias: None , )*
                    $( $win_alias: false , ),*
                    $( $e_alias: false ),*
                }
            }
        }


        /// An abstraction over Rust-SDL2's EventPump that also acts as a record of the
        /// application's events as-of the last time `Events::pump` was called.
        pub struct Events {
            pump: EventPump,
            pub now: ImmediateEvents,

            $( pub $k_alias: bool ),*
        }

        impl Events {
            /// Create a new event record based on some SDL event pump.
            pub fn new(pump: EventPump) -> Events {
                Events {
                    pump: pump,
                    now: ImmediateEvents::new(),

                    // By default, initialize every key with _not pressed_
                    $( $k_alias: false ),*
                }
            }

            /// Pump the events that happened since the last frame and update the events
            /// record accordingly.
            pub fn pump(&mut self) {
                self.now = ImmediateEvents::new();

                for event in self.pump.poll_iter() {
                    use sdl2::event::Event::*;
                    use sdl2::event::WindowEvent::Resized;
                    use sdl2::keyboard::Keycode::*;

                    match event {
                        KeyDown { keycode, .. } => match keycode {
                            $(
                                Some($k_sdl) => {
                                    // Prevent multiple presses when keeping a key down
                                    if !self.$k_alias {
                                        // Key pressed
                                        self.now.$k_alias = Some(true);
                                    }

                                    self.$k_alias = true;
                                }
                            ),*
                            _ => {}
                        },

                        KeyUp { keycode, .. } => match keycode {
                            $(
                                Some($k_sdl) => {
                                    // Key released
                                    self.now.$k_alias = Some(false);
                                    self.$k_alias = false;
                                }
                            ),*
                            _ => {}
                        },
                        Window { win_event, .. } => match win_event {
                            $(
                                $win_sdl => {
                                    self.now.$win_alias = true;
                                },
                            ),*

                            _ => {}
                        },

                        $(
                            $e_sdl => {
                                self.now.$e_alias = true;
                            },
                        )*

                        _ => {}
                    }
                }
            }
        }
    }
}
