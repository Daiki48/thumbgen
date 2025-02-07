#[derive(serde::Deserialize)]
pub struct Config {
    pub meta: Meta,
    pub design: Design,
}

#[derive(serde::Deserialize)]
pub struct Meta {
    pub title: String,
    pub username: String,
}

#[derive(serde::Deserialize)]
pub struct Design {
    pub background_color: Option<[u8; 4]>,
    pub box_color: Option<[u8; 4]>,
    pub font_color: Option<[u8; 4]>,
}

impl Design {
    pub fn get_rgba_background_color(&self) -> image::Rgba<u8> {
        let default_background_color: image::Rgba<u8> = image::Rgba([34, 39, 46, 255]);
        self.background_color
            .map_or(default_background_color, image::Rgba)
    }

    pub fn get_rgba_box_color(&self) -> image::Rgba<u8> {
        let default_box_color: image::Rgba<u8> = image::Rgba([55, 63, 74, 255]);
        self.box_color.map_or(default_box_color, image::Rgba)
    }

    pub fn get_rgba_font_color(&self) -> image::Rgba<u8> {
        let default_font_color: image::Rgba<u8> = image::Rgba([202, 204, 202, 255]);
        self.font_color.map_or(default_font_color, image::Rgba)
    }
}

pub const OUTPUT_DIR: &str = ".thumbgen";
pub const TOML_DEFAULT: &str = r#"
# This config is setup for thumbgen

[meta]
title = 'Hello. I love Rust language. Rust is best for programming. I writing Rust at thumbgen. My Github username is Daiki48.'
username = 'Daiki48'

[design]
background_color = [34, 39, 46, 255]
box_color = [55, 63, 74, 255]
font_color = [202, 204, 202, 255]
"#;

pub fn load_config() -> Result<Config, crate::error::ThumbnailError> {
    let config_path = format!("{}/config.toml", OUTPUT_DIR);
    let config_content = std::fs::read_to_string(&config_path).map_err(|e| {
        crate::error::ThumbnailError::Io(std::io::Error::new(
            e.kind(),
            format!("Failed to read config file: {}", e),
        ))
    })?;
    toml::from_str(&config_content).map_err(crate::error::ThumbnailError::Toml)
}
