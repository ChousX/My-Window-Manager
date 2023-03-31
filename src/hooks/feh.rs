use penrose::{core::hooks::StateHook, util::spawn, x::XConn};

pub enum Feh {
    Center,
}
impl<X: XConn> StateHook<X> for Feh {
    fn call(&mut self, state: &mut penrose::core::State<X>, x: &X) -> penrose::Result<()> {
        spawn("feh --no-fehbg --bg-scale ~/.config/feh/bg.png")
    }
}
