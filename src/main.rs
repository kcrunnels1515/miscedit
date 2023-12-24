use std::{time::Duration, io};
use crossterm::terminal::{enable_raw_mode,disable_raw_mode,ClearType::All};
use crossterm::{
    //execute,
    //style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    //ExecutableCommand, Result,
    //terminal::{Clear},
    event::{poll, read, Event, KeyEvent, KeyCode, KeyModifiers},
};

fn poll_insert(text: &mut Vec<char>, b_gap: &mut usize, e_gap: &mut usize, code: KeyCode, modifiers: KeyModifiers) -> io::Result<()> {
    match code {
        KeyCode::Char(chr) => {
            if chr == 'c' && modifiers == KeyModifiers::CONTROL {
                return Err(io::Error::other("oh no"));
            } else {
                text[*b_gap] = chr;
                *b_gap += 1;
            }
        },
        KeyCode::Backspace => {
            if *b_gap > 0 {
                //text[*b_gap - 1] = '\0';
                *b_gap -= 1;
            }
        },
        KeyCode::Left => {
            if *b_gap > 0 {
                text[*e_gap] = text[*b_gap-1];
                text[*b_gap-1] = '\0';
                *e_gap -= 1;
                *b_gap -= 1;
            }
        },
        KeyCode::Right => {
            if *e_gap < text.len() - 1  {
                text[*b_gap] = text[*e_gap+1];
                text[*e_gap] = '\0';
                *e_gap += 1;
                *b_gap += 1;
            }
        },
        _ => {},
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut text: Vec<char> = vec!['\0'; 50];
    enable_raw_mode()?;
    let mut e_gap: usize = 49;
    let mut b_gap: usize = 0;
    loop {
        if poll(Duration::from_millis(500))? {
            if let Event::Key(KeyEvent { code, modifiers, ..}) = read()? {
                if poll_insert(&mut text, &mut b_gap, &mut e_gap, code, modifiers).is_err() {
                    break;
                }
            }
        }
    }
    println!("{:?}", text);
    disable_raw_mode()?;
    Ok(())
}
