//-----------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub enum ArgsParseError {
    NoInputFilename,
    NoOutputFilename,
    HelpGiven,
}

impl std::fmt::Display for ArgsParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ArgsParseError::NoInputFilename =>  {
                return write!(f, "no input filename given");
            }
            ArgsParseError::NoOutputFilename => {
                return write!(f, "no output filename given");
            }
            ArgsParseError::HelpGiven => {
                return write!(f, "");
            }
        }
    }
}

impl std::fmt::Debug for ArgsParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self);
    }
}

impl std::error::Error for ArgsParseError {}

//-----------------------------------------------------------------------------

pub fn parse_args() -> Result<(), ArgsParseError> {
    let mut args = std::env::args();

    let mut has_input_filename = false;
    let mut has_output_filename = false;

    while let Some(arg) = args.next() {
        if arg == "-i" && !has_input_filename {
            if let Some(filename) = args.next() {
                let mut config = crate::CONFIG.lock().expect("Failed to get access to global config");
                config.input_filename = filename;
                has_input_filename = true;
            } else {
                return Err(ArgsParseError::NoInputFilename);
            }
        }
        if arg == "-o" && !has_output_filename {
            if let Some(filename) = args.next() {
                let mut config = crate::CONFIG.lock().expect("Failed to get access to global config");
                config.output_filename = filename;
                has_output_filename = true;
            } else {
                return Err(ArgsParseError::NoOutputFilename);
            }
        }
        if arg == "--help" {
            return Err(ArgsParseError::HelpGiven);
        }
    }

    if !has_input_filename {
        return Err(ArgsParseError::NoInputFilename);
    }
    if !has_output_filename {
        return Err(ArgsParseError::NoOutputFilename);
    }

    Ok(())
}

pub fn print_help() {
    println!("Usage: nova_cs -i 'INPUT_FILENAME' -o 'OUTPUT_FILENAME'");
}
