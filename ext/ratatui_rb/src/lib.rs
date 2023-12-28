use std::io::stdout;

use crossterm::{ExecutableCommand, terminal::{EnterAlternateScreen, enable_raw_mode, LeaveAlternateScreen, disable_raw_mode}};
use magnus::{define_module, function, prelude::*, Error};
use anyhow::{Ok, Result};
use ratatui::{Terminal, backend::CrosstermBackend};

fn hello() -> bool {
    let terminal = startup()?;
    let result = main_loop(terminal)?;
    finalize();

    if result.is_err() { return false; }

    true
}

fn startup() -> Result<Terminal<_>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    Ok(terminal)
}

fn main_loop(terminal: Terminal<_>) -> Result<()> {
    loop {

    }
}

fn finalize() -> Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("RatatuiRb")?;
    module.define_singleton_method("hello", function!(hello, 0))?;
    std::result::Result::Ok(())
}
