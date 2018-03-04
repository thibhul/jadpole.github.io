#[macro_use]
mod events;

use sdl2::render::Canvas;
use sdl2::video::Window;

// Generate an `Events` structure to record SDL events.
struct_events! {
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down,
        key_space: Space
    },
    else: {
        quit: Quit { .. }
    }
}


/// Bundles the Phi abstractions in a single structure which  can be passed
/// easily between functions.
pub struct Phi {
    pub events: Events,
    pub renderer: Canvas<Window>,
}


/// A `ViewAction` is a way for the currently executed view to communicate with
/// the game loop. It specifies which action should be executed before the next
/// rendering.
pub enum ViewAction {
    None,
    Quit,
    ChangeView(Box<View>),
}


/// Interface through which Phi interacts with the possible states in which the
/// application can be.
pub trait View {
    /// Called on every frame to take care of both the logic and
    /// the rendering of the current view.
    ///
    /// `elapsed` is expressed in seconds.
    fn render(&mut self, context: &mut Phi, elapsed: f64) -> ViewAction;
}

/// Create a window with name `title`, initialize the underlying libraries and
/// start the game with the `View` returned by `init()`.
///
/// # Examples
///
/// Here, we simply show a window with color #ffff00 and exit when escape is
/// pressed or when the window is closed.
///
/// ```
/// struct MyView;
///
/// impl View for MyView {
///     fn render(&mut self, context: &mut Phi, _: f64) -> ViewAction {
///         if context.events.now.quit {
///             return ViewAction::Quit;
///         }
///
///         context.renderer.set_draw_color(Color::RGB(255, 255, 0));
///         context.renderer.clear();
///         ViewAction::None
///     }
/// }
///
/// spawn("Example", |_| {
///     Box::new(MyView)
/// });
/// ```
pub fn spawn<F>(title: &str, init: F)
    where F: Fn(&mut Phi) -> Box<View>
{
    // Initialize SDL2
    let sdl_context = ::sdl2::init().expect("Could not initialize SDL2");
    let video = sdl_context.video().expect("Could not load the video component");
    let mut timer = sdl_context.timer().unwrap();

    // Open the main window
    let window = video.window(title, 800, 600)
        .position_centered()
        .opengl()
        .build()
        .expect("Could not open the main window");

    // Create the context
    let mut context = Phi {
        events: Events::new(sdl_context.event_pump().unwrap()),
        renderer: window.into_canvas()
            .accelerated()
            .build()
            .expect("Could not create a canvas for the main window"),
    };

    // Create the default view
    let mut current_view = init(&mut context);

    // Frame timing
    let interval = 1_000 / 60;
    let mut before = timer.ticks();
    let mut last_second = timer.ticks();
    let mut fps = 0u16;

    loop {
        // Frame timing (bis)

        let now = timer.ticks();
        let dt = now - before;
        let elapsed = dt as f64 / 1_000.0;

        // If the time elapsed since the last frame is too small, wait out the
        // difference and try again.
        if dt < interval {
            timer.delay(interval - dt);
            continue;
        }

        before = now;
        fps += 1;

        if now - last_second > 1_000 {
            println!("FPS: {}", fps);
            last_second = now;
            fps = 0;
        }


        // Logic & rendering

        context.events.pump();

        match current_view.render(&mut context, elapsed) {
            ViewAction::None => context.renderer.present(),

            ViewAction::Quit => break,

            ViewAction::ChangeView(new_view) => current_view = new_view,
        }
    }
}