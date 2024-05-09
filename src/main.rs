use std::error::Error;
use nova_cs::palette;

fn main() -> Result<(), Box<dyn Error>> {
    if let Err(e) = nova_cs::parse_args() {
        match e {
            nova_cs::ArgsParseError::HelpGiven => {
                nova_cs::print_help();
                return Ok(());
            }
            _ => {
                nova_cs::print_help();
            }
        }

        return Err(e.into());
    }

    let input_filename  = nova_cs::CONFIG.lock()?.input_filename.clone();
    let output_filename = nova_cs::CONFIG.lock()?.output_filename.clone();

    let buffer = std::fs::read(input_filename)?;
    let data = std::str::from_utf8(&buffer)?;

    let mut palette = palette::toml::load_palette(&data)?;
    palette::calculate_more_colors(&mut palette);

    let lua = palette::lua::palette_to_lua(&palette);

    std::fs::write(output_filename, &lua)?;

    Ok(())
}
