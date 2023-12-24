use std::{time::Duration, io};
use crossterm::terminal::{enable_raw_mode,disable_raw_mode,ClearType::All};
use crossterm::{
    //execute,
    //style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    //ExecutableCommand, Result,
    //terminal::{Clear},
    event::{poll, read, Event, KeyEvent, KeyCode, KeyModifiers},
};

fn poll_insert(text: &mut Vec<char>, ind: &mut usize, code: KeyCode, modifiers: KeyModifiers) -> io::Result<()> {
    if let KeyCode::Char(chr) = code {
        if chr == 'c' && modifiers == KeyModifiers::CONTROL {
            return Err(io::Error::other("oh no"));
        } else {
            text[*ind] = chr;
            *ind += 1;
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut text: Vec<char> = vec!['\0'; 50];
    enable_raw_mode()?;
    let mut insert_at: usize = 0;
    loop {
        if poll(Duration::from_millis(500))? {
            if let Event::Key(KeyEvent { code, modifiers, ..}) = read()? {
                if poll_insert(&mut text, &mut insert_at, code, modifiers).is_err() {
                    break;
                }
            }
        }
    }
    println!("{:?}", text);
    disable_raw_mode()?;
    Ok(())
}
