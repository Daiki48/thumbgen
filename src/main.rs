mod config;
mod error;
mod font;
mod image_process;

#[derive(clap::Parser)]
#[command(name = "thumbgen")]
#[command(about = "A simple CLI tool for generating thumbnails")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Initializes the configuration directory
    Init,
    /// Generates a thumbnail
    Create,
}

fn main() -> Result<(), error::ThumbnailError> {
    let cli = <Cli as clap::Parser>::parse();

    match &cli.command {
        Commands::Init => init(),
        Commands::Create => create(),
    }
}

fn init() -> Result<(), error::ThumbnailError> {
    std::fs::create_dir_all(config::OUTPUT_DIR).map_err(error::ThumbnailError::Io)?;
    std::fs::write(".thumbgen/config.toml", config::TOML_DEFAULT)
        .map_err(error::ThumbnailError::Io)?;
    println!("Initialized .thumbgen directory with config.toml file");
    Ok(())
}

fn create() -> Result<(), error::ThumbnailError> {
    let config = config::load_config()?;
    let font = font::load_font()?;
    image_process::generate_thumbnail(
        &config.metas.title,
        &config.metas.username,
        config.design.get_rgba_background_color(),
        config.design.get_rgba_font_color(),
        &font,
    )?;
    println!("Finished");

    Ok(())
}
