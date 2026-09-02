use t_glib::Terminal;
use t_glib::components::{ Label, Pixel, PixelSet };
use t_glib::colors::*;
use t_glib::types::Text;
use std::time::Duration;

fn main() {
    let terminal = Terminal::new();
    terminal.setup();
    
    let my_text = Label {
        pos: (40, 3),
        fore_color: GREEN,
        back_color: BG_BLACK,
        text: Text::from_str("Welcome to TgLib! | Version 0.2.1"),
    };

    let pixel_set = PixelSet {
        pixels: vec![
            Pixel { pos: (40, 5), color: YELLOW }, Pixel { pos: (41, 5), color: YELLOW },
            Pixel { pos: (42, 5), color: YELLOW }, Pixel { pos: (43, 5), color: YELLOW },
            Pixel { pos: (44, 5), color: YELLOW }, Pixel { pos: (45, 5), color: YELLOW },
            Pixel { pos: (46, 5), color: YELLOW }, Pixel { pos: (47, 5), color: YELLOW },
            Pixel { pos: (48, 5), color: YELLOW }, Pixel { pos: (49, 5), color: YELLOW },

            Pixel { pos: (40, 6), color: YELLOW }, Pixel { pos: (41, 6), color: YELLOW },
            Pixel { pos: (42, 6), color: BLACK }, Pixel { pos: (43, 6), color: BLACK },
            Pixel { pos: (44, 6), color: YELLOW }, Pixel { pos: (45, 6), color: YELLOW },
            Pixel { pos: (46, 6), color: BLACK }, Pixel { pos: (47, 6), color: BLACK },
            Pixel { pos: (48, 6), color: YELLOW }, Pixel { pos: (49, 6), color: YELLOW },

            Pixel { pos: (40, 7), color: YELLOW }, Pixel { pos: (41, 7), color: YELLOW },
            Pixel { pos: (42, 7), color: YELLOW }, Pixel { pos: (43, 7), color: YELLOW },
            Pixel { pos: (44, 7), color: YELLOW }, Pixel { pos: (45, 7), color: YELLOW },
            Pixel { pos: (46, 7), color: YELLOW }, Pixel { pos: (47, 7), color: YELLOW },
            Pixel { pos: (48, 7), color: YELLOW }, Pixel { pos: (49, 7), color: YELLOW },

            Pixel { pos: (40, 8), color: YELLOW }, Pixel { pos: (41, 8), color: YELLOW },
            Pixel { pos: (42, 8), color: BLACK }, Pixel { pos: (43, 8), color: BLACK },
            Pixel { pos: (44, 8), color: BLACK }, Pixel { pos: (45, 8), color: BLACK },
            Pixel { pos: (46, 8), color: BLACK }, Pixel { pos: (47, 8), color: BLACK },
            Pixel { pos: (48, 8), color: YELLOW }, Pixel { pos: (49, 8), color: YELLOW },

            Pixel { pos: (40, 9), color: YELLOW }, Pixel { pos: (41, 9), color: YELLOW },
            Pixel { pos: (42, 9), color: YELLOW }, Pixel { pos: (43, 9), color: YELLOW },
            Pixel { pos: (44, 9), color: YELLOW }, Pixel { pos: (45, 9), color: YELLOW },
            Pixel { pos: (46, 9), color: YELLOW }, Pixel { pos: (47, 9), color: YELLOW },
            Pixel { pos: (48, 9), color: YELLOW }, Pixel { pos: (49, 9), color: YELLOW },
        ],
    };

    loop {
        terminal.printc(&my_text);
        terminal.show_pixelset(&pixel_set);
        terminal.hide_cursor();

        if terminal.is_interrupted() {
            let interrupt_text = Label {
                pos: (40, 4),
                fore_color: RED,
                back_color: BG_BLACK,
                text: Text::from_str("Exiting Program"),
            };

            terminal.printc(&interrupt_text);
            terminal.sleep(Duration::from_secs(2));
            terminal.hide_cursor();
            terminal.leave();
            break;
        }

        terminal.sleep(Duration::from_millis(33));
    }
}