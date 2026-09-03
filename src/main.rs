use t_glib::Terminal;
use t_glib::components::{Label, BIND};
use t_glib::colors::*;
use t_glib::types::{Text, Key};
use std::time::Duration;
use std::sync::atomic::{ AtomicUsize, Ordering };

static COUNT: AtomicUsize = AtomicUsize::new(0);

fn inc_count() {
    COUNT.fetch_add(1, Ordering::Relaxed);
}

fn main() {
    let mut terminal = Terminal::new();

    terminal.setup();

    let my_text = Label {
        pos: (40, 3),
        fore_color: GREEN,
        back_color: BG_BLACK,
        text: Text::from_str(
            "Welcome to TgLib! | Version 0.2.3"
        ),
    };

    let counter_bind = BIND {
        func: inc_count,
        key: Key::SPACE,
    };

    terminal.bind(counter_bind);

    loop {
        terminal.process_binds();
        terminal.printc(&my_text);

        let count: usize = COUNT.load(Ordering::Relaxed);
        let counter_string: String = format!("Count: {}     ",count);

        let counter_text = Label {
            pos: (40, 4),
            fore_color: YELLOW,
            back_color: BG_BLACK,
            text: Text::from_str(&counter_string),
        };

        terminal.printc(&counter_text);
        terminal.hide_cursor();

        if terminal.is_interrupted() {
            let interrupt_text = Label {
                pos: (40, 5),
                fore_color: RED,
                back_color: BG_BLACK,
                text: Text::from_str(
                    "Exiting Program"
                ),
            };

            terminal.printc(&interrupt_text);
            terminal.sleep(Duration::from_secs(2));
            terminal.leave();

            break;
        }

        terminal.sleep(Duration::from_millis(16));
    }
}