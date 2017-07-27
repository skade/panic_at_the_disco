extern crate open;

pub fn init() {
    std::panic::set_hook(Box::new(|panic_info| {
        if Some(&"at the disco") == panic_info.payload().downcast_ref::<&str>() {
           open::that("https://www.youtube.com/watch?v=H5NqIsnyTG8").ok();
        }
    }));
}