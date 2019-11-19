use std::io;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

mod zap_parser;
mod go_test_parser;

struct Colors {
    logger: ColorSpec,
    error: ColorSpec,
    warn: ColorSpec,
    log: ColorSpec,
    info: ColorSpec,
    debug: ColorSpec,
    message: ColorSpec,
    key: ColorSpec,
    value: ColorSpec,
    punc: ColorSpec,
    muted: ColorSpec,
}

impl Colors {
    fn defaults() -> Colors {
        let mut logger = ColorSpec::new();
        logger.set_fg(Some(Color::Ansi256(32u8)));

        let mut info = ColorSpec::new();
        info.set_fg(Some(Color::Blue));

        let mut error = ColorSpec::new();
        error.set_fg(Some(Color::Red)).set_bold(true);

        let log = ColorSpec::new();

        let mut debug = ColorSpec::new();
        debug.set_fg(Some(Color::Green));

        let mut warn = ColorSpec::new();
        warn.set_fg(Some(Color::Red)).set_bold(true);

        // Referencing colors from https://jonasjacek.github.io/colors/

        let mut message = ColorSpec::new();
        message.set_fg(Some(Color::Ansi256(252u8)));

        let mut key = ColorSpec::new();
        key.set_fg(Some(Color::Ansi256(249u8)));

        let mut value = ColorSpec::new();
        value.set_fg(Some(Color::Ansi256(122u8)));

        let mut punc = ColorSpec::new();
        punc.set_fg(Some(Color::Ansi256(102u8)));

        let mut muted = ColorSpec::new();
        muted.set_fg(Some(Color::Ansi256(242u8)));

        return Colors {
            logger,
            error,
            warn,
            log,
            info,
            debug,
            message,
            key,
            value,
            punc,
            muted,
        };
    }
}

trait LogParser {
    fn add(&mut self, stdout: &mut termcolor::StandardStream, colors: &Colors, line: &str) -> io::Result<()>;
}

fn main() -> Result<(), ()> {
    let mut input = String::new();
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut stderr = StandardStream::stderr(ColorChoice::Always);
    let colors = Colors::defaults();
    let stdin = io::stdin();
    let mut zap_parser = zap_parser::ZapParser::new();
    let mut go_test_parser = go_test_parser::GoTestParser::new();
    loop {
        match stdin.read_line(&mut input) {
            Ok(0) => return Ok(()),
            Ok(_number_of_bytes_read) => {
                match zap_parser.add(&mut stdout, &colors, &input) {
                    Ok(_) => {}
                    Err(err) => {
                        let show_errors_opt = std::env::var("ZAP_PRETTIER_SHOW_ERRORS").ok();
                        if let Some(show_errors) = show_errors_opt {
                            if show_errors.as_str() == "1" {
                                stderr.set_color(&colors.error).unwrap();
                                writeln!(&mut stderr, "Error printing \"{}\": {:?}", input, err)
                                .unwrap();
                            }
                        }
                        match go_test_parser.add(&mut stdout, &colors, &input) {
                            Ok(_) => {}
                            Err(_) => {
                                stderr.set_color(&colors.muted).unwrap();
                                write!(&mut stdout, "{}", input).unwrap();
                                stderr.reset().unwrap();
                            }
                        }
                    }
                };
                input.clear()
            }
            Err(error) => eprintln!("error: {}", error),
        }
    }
}
