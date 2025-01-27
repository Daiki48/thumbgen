#[derive(thiserror::Error, Debug)]
pub enum ThumbnailError {
    #[error("Font loading failed")]
    FontLoad,
    #[error("Image saving failed : {0}")]
    ImageSave(#[from] image::ImageError),
    #[error("Io failed : {0}")]
    Io(#[from] std::io::Error),
    #[error("Toml failed : {0}")]
    Toml(#[from] toml::de::Error),
}
