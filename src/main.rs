//-----------------------------------------------------------------------------
use clap::Parser;
use nova_cs::palette;
use std::error::Error;
//-----------------------------------------------------------------------------

#[derive(Parser, Debug)]
#[command(name = "nova_cs")]
#[command(about = "Generate NovaCS theme palette based on just 14 colors", long_about = None)]
#[command(version)]
pub struct CliArgs {
    /// Input TOML file path
    #[arg(short, long, value_name = "INPUT_FILENAME")]
    pub input: String,

    /// Output lua file path
    #[arg(short, long, value_name = "OUTPUT_FILENAME")]
    pub output: String,
}

//-----------------------------------------------------------------------------

fn main() -> Result<(), Box<dyn Error>> {
    let args = CliArgs::parse();

    let buffer = std::fs::read(&args.input)?;
    let data = std::str::from_utf8(&buffer)?;

    let mut palette = palette::toml::load_palette(&data)?;
    palette::calculate_more_colors(&mut palette);

    let lua = palette::lua::palette_to_lua(&palette);

    std::fs::write(&args.output, &lua)?;

    Ok(())
}

//-----------------------------------------------------------------------------
