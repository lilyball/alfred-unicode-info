#[crate_id="com.tildesoft.alfred.workflow.unicode/unicode#0.1"];
#[crate_type="bin"];

use std::char;
use std::os;
use std::num;
use std::io;
use std::str;

#[allow(dead_code)]
mod icu;

#[allow(dead_code)]
#[warn(missing_doc)]
mod alfred;

fn main() {
    let args = os::args();

    if args.len() != 2 {
        println!("usage: {} text", args[0]);
        os::set_exit_status(2);
        return;
    }

    let text = args[1];
    let _ = handleArg(text);
}

/// Handles the given arg
fn handleArg(text: &str) -> io::IoResult<()> {
    if text.starts_with("U+") && text.len() > 2 && text.len() < 10 {
        let digits = text.slice_from(2);
        match num::from_str_radix::<u32>(digits, 16) {
            None => (),
            Some(code) => {
                // this is a U+#### codepoint
                if if_ok!(handleCodepoint(code)) {
                    return Ok(());
                }
            }
        }
    }
    handleText(text)
}

/// Prints out the XML for the given codepoint, if valid.
/// Returns `Ok(true)` if the codepoint is valid, `Ok(false)` if not.
fn handleCodepoint(code: u32) -> io::IoResult<bool> {
    let name = match icu::u_charName(code, icu::U_EXTENDED_CHAR_NAME) {
        Ok(~"") => ~"<unknown>",
        Ok(name) => name,
        Err(e) => {
            let _ = writeln!(&mut io::stderr(), "u_charName error: {}", e);
            return Ok(false);
        }
    };

    let mut stdout = io::stdout();

    if_ok!(stdout.write_str(XML_HEADER));

    let title = str::from_char(char::from_u32(code).unwrap_or('\uFFFD'));
    let subtitle = format!("U+{:04X} {}", code, name);

    let item = alfred::Item {
        arg: Some(title.clone().into_maybe_owned()),
        subtitle: Some(subtitle.into_maybe_owned()),
        icon: Some(alfred::PathIcon("icon.png".into_maybe_owned())),
        ..alfred::Item::new(title)
    };
    if_ok!(item.write_xml(&mut stdout, 1));

    if_ok!(stdout.write_str(XML_FOOTER));
    if_ok!(stdout.flush());
    Ok(true)
}

/// Prints out the XML for the sequence of characters.
fn handleText(text: &str) -> io::IoResult<()> {
    let mut stdout = io::stdout();

    if_ok!(stdout.write_str(XML_HEADER));

    for c in text.chars() {
        let name = match icu::u_charName(c as u32, icu::U_EXTENDED_CHAR_NAME) {
            Ok(~"") => ~"<unknown>",
            Ok(name) => name,
            Err(e) => {
                let _ = writeln!(&mut io::stderr(), "u_charName error: {}", e);
                continue;
            }
        };
        let item = alfred::Item {
            arg: Some(format!("U+{:04X} {}", c as u32, name).into_maybe_owned()),
            subtitle: Some(format!("U+{:04X}", c as u32).into_maybe_owned()),
            icon: Some(alfred::PathIcon("icon.png".into_maybe_owned())),
            ..alfred::Item::new(name)
        };
        if_ok!(item.write_xml(&mut stdout, 1));
    }

    if_ok!(stdout.write_str(XML_FOOTER));
    if_ok!(stdout.flush());
    Ok(())
}

static XML_HEADER: &'static str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<items>\n";
static XML_FOOTER: &'static str = "</items>\n";
