use std::error::Error;

slint::include_modules!();


pub fn gui() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?; 
    ui.run()?; 

    Ok(())
}
