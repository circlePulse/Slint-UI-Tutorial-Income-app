slint::include_modules!();

const TAXPER: f64 = 0.3;
const OWNERPER: f64 = 0.55;
const PROFITPER: f64 = 0.05;
const OPEXPER: f64 = 0.10;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    ui.on_divide_income({
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    ui.run()
}