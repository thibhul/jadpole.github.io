## 1-1

* RendererBuilder -> CanvasBuilder
* window.renderer() -> window.into_canvas()
* sdl2 : 0.16.1 -> 0.30.0

## 1-2

Same 1-1, rename `'p` to `ev_ctx`, to take the habit to name the lifetimes.

## 1-3

Same 1-1

## 1-4

* Same 1-1
* No more lifetime in Phi, Canvas uses now Window as a type argument instead of a lifetime

## 1-5

* Same 1-4

## 1-6

* Same 1-4
* No more Resized event, set in WindowEvent::Resized and change macro to handle WindowEvent

## 1-7

* Same 1-4
* Resize need to be changed, type and expected behaviour added to macro_events
* Sdl2_image is a feature now, change cargo.toml
* Texture has now a lifetime (should be the starting point to explain lifetimes ?)
* Change every Renderer to Canvas<Window>
* Rect::new always succeeds