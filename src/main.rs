use t_glib::Terminal;
use t_glib::components::Label;
use t_glib::colors::*;
use t_glib::types::Text;

fn main(){
    let terminal = Terminal::new();

    terminal.setup();

    let my_text = Label {
        pos: (40, 3),
        fore_color: GREEN,
        back_color: BG_BLACK,
        text: Text::from_str("Welcome to TgLib! | Version 0.1.0"),
    };

    terminal.printc(&my_text);

    loop {
        if terminal.is_interrupted() {
            let interrupt_text = Label {
                pos: (40, 4),
                fore_color: RED,
                back_color: BG_BLACK,
                text: Text::from_str("Exiting Program"),
            };

            terminal.printc(&interrupt_text);
            terminal.sleep(std::time::Duration::from_secs(2));
            terminal.leave();
        }
    }
}