// ref: // ref: https://stackoverflow.com/questions/30291757/attaching-an-icon-resource-to-a-rust-application/65393488#65393488
use std::{env, io};

#[cfg(target_family = "windows")]
fn main() -> io::Result<()> {
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        use winres::WindowsResource;

        WindowsResource::new()
            // This path can be absolute, or relative to your crate root.
            .set_icon("src//static//icon.ico")
            .compile()?;
    }
    Ok(())
}

#[cfg(target_family = "unix")]
fn main() {
    println!("aaaa");
}
