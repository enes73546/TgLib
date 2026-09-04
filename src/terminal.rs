#[cfg(target_os = "windows")]
use crate::platform::windows::*;

#[cfg(unix)]
use crate::platform::unix::*;

use crate::components::{Label, NCLabel, BIND};
use crate::colors::RESET;
use std::io::{self, Write};
use crate::colors::*;
use crate::types::Key;

pub struct Terminal {
    width: u16,
    height: u16,
    pub binds: Vec<BIND>,
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            width: get_terminal_width(),
            height: get_terminal_height(),
            binds: Vec::new(),
        }
    }

    pub fn sleep(&self, duration: std::time::Duration) {
        std::thread::sleep(duration);
    }

    pub fn bind(&mut self, bind: BIND) {
        self.binds.push(bind);
    }

    pub fn handle_key(&self, key: Key) {
        for bind in &self.binds {
            if bind.key == key {
                (bind.func)();
            }
        }
    }

    pub fn process_binds(&self) {
        if let Some(key) = read_key() {
            self.handle_key(key);
        }
    }

    fn enter(&self) {
        print!("\x1b[?1049h");
        print!("\x1b[2J");
        print!("\x1b[H");

        io::stdout().flush().unwrap();
    }

    pub fn leave(&self) {
        restore_keyboard();

        print!("\x1b[?25h");
        print!("\x1b[?1049l");

        io::stdout().flush().unwrap();
    }

    pub fn clear_screen(&self) {
        clear_screen();
        io::stdout().flush().unwrap();
    }

    fn install_ctrl_c_handler(&self) {
        install_ctrl_c_handler();
    }

    pub fn setup(&self) {
        self.enter();
        self.install_ctrl_c_handler();
        setup_keyboard();
    }

    pub fn is_interrupted(&self) -> bool {
        is_interrupted()
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn debug_coordinates(&self) {
        self.debug_coordinates_x();
        self.debug_coordinates_y();
    }

    pub fn debug_coordinates_y(&self) {
        for y in 1..=self.height {
            if y < 10 {
                print!(
                    "\x1b[{};1H{}0{}~{}",
                    y,
                    B_CYAN,
                    y,
                    RESET
                );
            } else {
                print!(
                    "\x1b[{};1H{}{}~{}",
                    y,
                    B_CYAN,
                    y,
                    RESET
                );
            }
        }

        io::stdout().flush().unwrap();
    }

    pub fn debug_coordinates_x(&self) {
        let pattern = format!(
            "{I_CYAN}0123456789{RESET}"
        );

        let count = (self.width + 9) / 10;

        print!(
            "\x1b[1;1H{}",
            pattern.repeat(count.into())
        );

        io::stdout().flush().unwrap();
    }

    pub fn print(&self, text: &NCLabel) {
        print!(
            "{}{}{}{}",
            text.fore_color,
            text.back_color,
            text.text.get_val(),
            RESET
        );

        io::stdout().flush().unwrap();
    }

    pub fn printc(&self, text: &Label) {
        print!(
            "\x1b[{};{}H{}{}{}{}",
            text.pos.1,
            text.pos.0,
            text.fore_color,
            text.back_color,
            text.text.get_val(),
            RESET
        );

        io::stdout().flush().unwrap();
    }

    pub fn hide_cursor(&self) {
        print!("\x1b[?25l");

        std::io::stdout()
            .flush()
            .unwrap();
    }

    pub fn show_cursor(&self) {
        print!("\x1b[?25h");

        std::io::stdout()
            .flush()
            .unwrap();
    }

    pub fn show_cell(
        &self,
        cell: &crate::components::Cell
    ) {
        print!(
            "\x1b[{};{}H{}{}{}{}",
            cell.pos.1,
            cell.pos.0,
            cell.color,
            cell.char_color,
            cell.char_,
            RESET
        );

        io::stdout().flush().unwrap();
    }

    pub fn show_pixel(
        &self,
        pixel: &crate::components::Pixel
    ) {
        print!(
            "\x1b[{};{}H{}{}{}",
            pixel.pos.1,
            pixel.pos.0,
            pixel.color,
            ' ',
            RESET
        );

        io::stdout().flush().unwrap();
    }

    pub fn show_pixelset(
        &self,
        pixel_set: &crate::components::PixelSet
    ) {
        print!("{}", pixel_set);

        io::stdout().flush().unwrap();
    }
}