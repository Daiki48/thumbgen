pub const BACKGROUND_WIDTH_SIZE: u32 = 1200;
pub const BACKGROUND_HEIGHT_SIZE: u32 = 630;

pub const BOX_WIDTH_SIZE: u32 = 1140;
pub const BOX_HEIGHT_SIZE: u32 = 600;
pub const BOX_COLOR: image::Rgba<u8> = image::Rgba([55, 63, 74, 255]);

pub const TITLE_FONT_SIZE: f32 = 52.0;
pub const USERNAME_FONT_SIZE: f32 = 38.0;

const FULL_SIZE_FILE_NAME: &str = "thumbgen_1200_630.png";

pub fn generate_thumbnail(
    title: &str,
    username: &str,
    background_color: image::Rgba<u8>,
    font_color: image::Rgba<u8>,
    font: &ab_glyph::FontRef,
) -> Result<(), crate::error::ThumbnailError> {
    let background_width = BACKGROUND_WIDTH_SIZE;
    let background_height = BACKGROUND_HEIGHT_SIZE;

    let box_width = BOX_WIDTH_SIZE;
    let box_height = BOX_HEIGHT_SIZE;
    let box_position_x: i32 = (background_width / 2 - box_width / 2) as i32;
    let box_position_y: i32 = (background_height / 2 - box_height / 2) as i32;

    let title_text_position_x: i32 = box_position_x + 20;
    let mut title_text_position_y: i32 = box_position_y + 80;

    let username_text_position_x: i32 = title_text_position_x + 940;
    let username_text_position_y: i32 = title_text_position_y + 420;

    let mut img = image::RgbaImage::new(background_width, background_height);

    imageproc::drawing::draw_filled_rect_mut(
        &mut img,
        imageproc::rect::Rect::at(0, 0).of_size(background_width, background_height),
        background_color,
    );

    imageproc::drawing::draw_filled_rect_mut(
        &mut img,
        imageproc::rect::Rect::at(box_position_x, box_position_y).of_size(box_width, box_height),
        BOX_COLOR,
    );

    let title_scale = ab_glyph::PxScale::from(TITLE_FONT_SIZE);
    let username_scale = ab_glyph::PxScale::from(USERNAME_FONT_SIZE);

    let wrapped_title = textwrap::wrap(title, 40);

    for line in wrapped_title {
        imageproc::drawing::draw_text_mut(
            &mut img,
            font_color,
            title_text_position_x,
            title_text_position_y,
            title_scale,
            &font,
            &line,
        );
        title_text_position_y += 60;
    }

    imageproc::drawing::draw_text_mut(
        &mut img,
        font_color,
        username_text_position_x,
        username_text_position_y,
        username_scale,
        &font,
        username,
    );

    img.save(format!(
        "{}/{}",
        crate::config::OUTPUT_DIR,
        FULL_SIZE_FILE_NAME
    ))
    .map_err(crate::error::ThumbnailError::ImageSave)?;

    Ok(())
}
