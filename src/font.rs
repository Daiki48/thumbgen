pub fn load_font() -> Result<ab_glyph::FontRef<'static>, crate::error::ThumbnailError> {
    let font_data = include_bytes!("../font/KosugiMaru-Regular.ttf");
    ab_glyph::FontRef::try_from_slice(font_data).map_err(|_| crate::error::ThumbnailError::FontLoad)
}
