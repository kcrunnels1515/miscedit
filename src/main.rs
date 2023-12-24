use std::{time::Duration, io};
use crossterm::terminal::{enable_raw_mode,disable_raw_mode,ClearType::All};
use crossterm::{
    //execute,
    //style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    //ExecutableCommand, Result,
    //terminal::{Clear},
    event::{poll, read, Event, KeyEvent, KeyCode, KeyModifiers},
};

fn poll_insert(text: &mut Vec<char>, b_gap: &mut usize, e_gap: &mut usize, code: KeyCode, modifiers: KeyModifiers) -> io::Result<bool> {
    let mut text_changed: bool = false;
    match code {
        KeyCode::Char(chr) => {
            if chr == 'c' && modifiers == KeyModifiers::CONTROL {
                return Err(io::Error::other("oh no"));
            } else {
                text[*b_gap] = chr;
                *b_gap += 1;
                text_changed = true;
            }
        },
        KeyCode::Backspace => {
            if *b_gap > 0 {
                //text[*b_gap - 1] = '\0';
                *b_gap -= 1;
                text_changed = true;
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
    Ok(text_changed)
}

fn buf_to_string(gap_buf: &Vec<char>, b_gap: usize, e_gap: usize) -> String {
    String::from_iter(&gap_buf[0..b_gap]) + &String::from_iter(&gap_buf[e_gap..(gap_buf.len() - 1)])
}

fn main() -> std::io::Result<()> {
    let mut text: Vec<char> = vec!['\0'; 50];
    enable_raw_mode()?;
    let mut e_gap: usize = 49;
    let mut b_gap: usize = 0;
    let mut dis_str: String = String::new();
    loop {
        if poll(Duration::from_millis(500))? {
            if let Event::Key(KeyEvent { code, modifiers, ..}) = read()? {
                let result: io::Result<bool> = poll_insert(&mut text, &mut b_gap, &mut e_gap, code, modifiers);
                if result.is_err() {
                    break;
                }
                if let Ok(change) = result {
                    if change {
                        dis_str = buf_to_string(&text,b_gap,e_gap);
                    }
                }
            }

        }
    }
    println!("{:?}", text);
    disable_raw_mode()?;
    Ok(())
}
