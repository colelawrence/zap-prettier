use crate::Colors;
use crate::LogParser;
use std::io;
use std::io::Write;
use termcolor::{WriteColor};

pub struct GoTestParser {}

impl GoTestParser {
    pub fn new() -> Self {
        GoTestParser{}
    }
}

impl LogParser for GoTestParser {
    fn add(
        &mut self,
        mut stdout: &mut termcolor::StandardStream,
        colors: &Colors,
        line: &str,
    ) -> io::Result<()> {
        // parse string
        let start = line.trim_start();
        if start.starts_with("--- FAIL:") {
            // print with colors
            stdout.set_color(&colors.error)?;
            write!(&mut stdout, "{}", line)?;
        } else if start.starts_with("=== RUN") {
            // print with colors
            stdout.set_color(&colors.debug)?;
            write!(&mut stdout, "{}", line)?;
        } else {
            return Err(io::Error::from(io::ErrorKind::InvalidData))
        }
        // reset
        stdout.reset()?;
        writeln!(&mut stdout, "")?;
        Ok(())
    }   
}
