use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::io::Write;
use std::io;

struct Colors {
    error: ColorSpec,
    warn: ColorSpec,
    log: ColorSpec,
    info: ColorSpec,
    debug: ColorSpec,
    message: ColorSpec,
    key: ColorSpec,
    value: ColorSpec,
    punc: ColorSpec,
    reset: ColorSpec,
}

impl Colors {
    fn defaults() -> Colors {
        let mut info = ColorSpec::new();
        info.set_fg(Some(Color::Blue));

        let mut error = ColorSpec::new();
        error.set_fg(Some(Color::Red)).set_bold(true);

        let log = ColorSpec::new();

        let mut debug = ColorSpec::new();
        debug.set_fg(Some(Color::Green));

        let mut warn = ColorSpec::new();
        warn.set_fg(Some(Color::Red)).set_bold(true);

        let mut message = ColorSpec::new();
        message.set_fg(Some(Color::Cyan));

        let mut key = ColorSpec::new();
        key.set_fg(Some(Color::Ansi256(249u8))).set_underline(true);

        let mut value = ColorSpec::new();
        value.set_fg(Some(Color::Ansi256(122u8)));

        let mut punc = ColorSpec::new();
        punc.set_fg(Some(Color::Ansi256(102u8)));

        let reset = ColorSpec::new();

        return Colors {
            error,
            warn,
            log,
            info,
            debug,
            message,
            key,
            value,
            punc,
            reset,
        }
    }
}

fn main() {
    let mut input = String::new();
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let colors = Colors::defaults();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                prettier(&mut stdout, &colors, &input).unwrap();
                input.clear()
            }
            Err(error) => println!("error: {}", error),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Log {
    level: String,
    message: String,
    #[serde(flatten)]
    rest: HashMap<String, Value>,
}


fn prettier(mut stdout: &mut termcolor::StandardStream, colors: &Colors, line: &str) -> io::Result<()> {
    let log = serde_json::from_str::<Log>(&line).unwrap();

    match log.level.as_str() {
        "error" => stdout.set_color(&colors.error)?,
        "warning" => stdout.set_color(&colors.warn)?,
        "info" => stdout.set_color(&colors.info)?,
        "debug" => stdout.set_color(&colors.debug)?,
        "log" => stdout.set_color(&colors.log)?,
        _ => stdout.reset()?,
    }
    write!(&mut stdout, "{}\t", log.level)?;

    stdout.set_color(&colors.message)?;
    write!(&mut stdout, "{}\t", log.message)?;
    
    let mut first = true;
    for (key, val) in log.rest.iter() {
        if first {
            first = false;
        } else {
            stdout.set_color(&colors.punc)?;
            write!(&mut stdout, ", ")?;
        }

        stdout.set_color(&colors.key)?;
        write!(&mut stdout, "{}", key)?;
        stdout.set_color(&colors.punc)?;
        write!(&mut stdout, "=")?;
        stdout.set_color(&colors.value)?;
        write!(&mut stdout, "{}", val.to_string())?;
    }
    stdout.set_color(&colors.reset)?;
    writeln!(&mut stdout, "")?;

    Ok(())
}
