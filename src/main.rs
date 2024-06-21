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
    style::{Print}
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::io::{stdout, Result, Error};
use std::fs;
use std::path::Path;


fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    
    // Main Loop
    loop {
        
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new("Hello Ratatui (press 'q' to quiet)")
                    .white().on_red(),
                area,
            );
        })?;
        
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
        
    }
    
    
    stdout()
        .execute(Print("sum:\n".to_string()))?
        .execute(Print(format!("1 + 1= {} ", 1 + 1)))?;

    
    stdout().execute(EnterAlternateScreen)?;
    disable_raw_mode()?;
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

