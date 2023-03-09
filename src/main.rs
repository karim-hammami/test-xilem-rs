use druid::{AppLauncher, PlatformError, WindowDesc};
use ui::ui_builder;

mod ui;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder());
    let data = 0_u32;
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(data)
}
