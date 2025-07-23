mod app;
mod components;
mod pages;
mod utils;

use anyhow::Result;
use leptos::prelude::*;

use crate::app::App;

fn main() -> Result<()> {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(App);
    Ok(())
}
