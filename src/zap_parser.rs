use crate::Colors;
use crate::LogParser;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::io;
use std::io::Write;
use termcolor::{WriteColor};

#[derive(Serialize, Deserialize, Debug)]
struct Log {
    level: String,
    message: String,
    logger: Option<String>,
    #[serde(flatten)]
    rest: HashMap<String, Value>,
}

pub struct ZapParser {}

impl ZapParser {
    pub fn new() -> Self {
        ZapParser{}
    }
}

impl LogParser for ZapParser {
    fn add(
        &mut self,
        mut stdout: &mut termcolor::StandardStream,
        colors: &Colors,
        line: &str,
    ) -> io::Result<()> {
        let log = serde_json::from_str::<Log>(&line)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
        
        if let Some(logger_name) = log.logger {
            stdout.set_color(&colors.logger)?;
            write!(&mut stdout, "{}", logger_name)?;
            stdout.set_color(&colors.punc)?;
            write!(&mut stdout, ":\t")?;
        }
    
        match log.level.as_str() {
            "error" => stdout.set_color(&colors.error)?,
            "warn" => stdout.set_color(&colors.warn)?,
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
            write!(&mut stdout, ":")?;
            stdout.set_color(&colors.value)?;
            write!(&mut stdout, "{}", val.to_string())?;
        }
        stdout.reset()?;
        writeln!(&mut stdout, "")?;
    
        Ok(())
    }   
}
