extern crate sdl2;

mod phi;
mod views;


fn main() {
    /// Interesting case of lifetimes
    ::phi::spawn("ArcadeRS Shooter",
                 |mut phi: &mut ::phi::Phi| Box::new(::views::ShipView::new(&mut phi)));
}