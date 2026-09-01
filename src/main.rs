use t_glib::Terminal;
use t_glib::components::Label;
use t_glib::colors::*;
use t_glib::types::Text;

fn main() {
    let terminal = Terminal::new();

    terminal.enter();
    terminal.clear_screen();

    let my_text = Label {
        pos: (40, 3),
        fore_color: GREEN,
        back_color: BG_BLACK,
        text: Text::from_str("Welcome to TgLib! | Version 0.1.0"),
    };

    terminal.printc(&my_text);

    terminal.sleep(std::time::Duration::from_hours(24)); // This keeps the prompt out your way until ^C
    terminal.leave();
}