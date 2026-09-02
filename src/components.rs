use crate::types::Text;

pub struct Label {
    pub pos: (u16, u16),
    pub fore_color: &'static str,
    pub back_color: &'static str,
    pub text: Text,
}

impl Label {
    pub fn to_nc(&self) -> NCLabel {
        NCLabel {
            fore_color: self.fore_color,
            back_color: self.back_color,
            text: self.text.clone(),
        }
    }
}

pub struct NCLabel {
    pub fore_color: &'static str,
    pub back_color: &'static str,
    pub text: Text,
}

impl NCLabel {
    pub fn to_label(&self, pos_: (u16, u16)) -> Label {
        Label {
            pos: pos_,
            fore_color: self.fore_color,
            back_color: self.back_color,
            text: self.text.clone(),
        }
    }
}

pub struct Cell {
    pub pos: (u16, u16),
    pub color: &'static str,
    pub has_char: bool,
    pub char: char,
    pub char_color: &'static str,
}


pub struct Pixel {
    pub pos: (u16, u16),
    pub color: &'static str,
}

pub struct PixelSet {
    pub pixels: Vec<Pixel>,
}

impl std::fmt::Display for PixelSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for pixel in &self.pixels {
            write!(
                f,
                "\x1b[{};{}H{}█\x1b[0m",
                pixel.pos.1,
                pixel.pos.0,
                pixel.color
            )?;
        }

        Ok(())
    }
}