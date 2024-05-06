use image::RgbImage;

pub struct SendItem {
    pub panel_image: RgbImage,
    pub equip: String,
    pub rarity: usize,
    pub lock: bool,
    pub discard: bool,
}