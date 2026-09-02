use t_glib::Terminal;
use t_glib::components::Label;
use t_glib::colors::*;
use t_glib::types::Text;
use std::time::Duration;

fn main() {
    let terminal = Terminal::new();
    terminal.setup();

    loop {
        // This code demo replaces the Usual Welcome Screen with the new 
        // demonstration of the coordinate debug functions to help you plot your graphics in the terminal. The coordinate debug functions are a new feature that allows you to see the coordinates of the terminal in real-time, making it easier to position your graphics accurately.
        
        terminal.hide_cursor();

        /*
        terminal.debug_coordinates_x();
        terminal.debug_coordinates_y();
        
        .debug_coordinates() shows both x and y lines
        */

        terminal.debug_coordinates();

        if terminal.is_interrupted() {
            let interrupt_text = Label {
                pos: (6, 3),
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