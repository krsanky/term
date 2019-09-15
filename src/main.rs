use std::io::{stdout, Write};
use termion::raw::IntoRawMode;

fn rainbow<W: Write>(stdout: &mut W, blue: u8) {
    write!(
        stdout,
        "{}{}",
        termion::cursor::Goto(1, 1),
        termion::clear::All
    )
    .unwrap();

    for red in 0..32 {
        let red = red * 8;
        for green in 0..64 {
            let green = green * 4;
            write!(
                stdout,
                "{} ",
                termion::color::Bg(termion::color::Rgb(red, green, blue))
            )
            .unwrap();
        }
        write!(stdout, "\n\r").unwrap();
    }

    write!(stdout, "{}", termion::style::Reset).unwrap();
}

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let blue = 172u8;
    rainbow(&mut stdout, blue);
}
