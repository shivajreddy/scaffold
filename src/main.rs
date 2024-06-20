/* 
    Project Name: scaffold
    Tui: Ratatui
*/

#![allow(unused)]

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode,
               EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::io::{stdout, Result, Error};
use std::fs;
use std::path::Path;


fn main() -> Result<()> {
    // Read the directory
    // let target_path = Path::new(r"C:\Users\sreddy\Desktop");
    // visit_path(&target_path);
    // 
    // stdout().execute(EnterAlternateScreen);
    // enable_raw_mode();

    println!("hi");

    Ok(())
}


fn visit_path(dir: &Path) -> Result<()> {
    if !dir.is_dir() {
        return Err(Error::other("It's not a valid directory"));
    }
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        println!("{} {path:?}", path.is_dir());
    }
    Ok(())
}

