use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};

static INTERRUPTED: AtomicBool = AtomicBool::new(false);

unsafe extern "C" fn ctrl_c_handler(_: libc::c_int) {
    INTERRUPTED.store(true, Ordering::SeqCst);
}

pub fn install_ctrl_c_handler() {
    unsafe {
        libc::signal(libc::SIGINT, Some(ctrl_c_handler));
    }
}

pub fn is_interrupted() -> bool {
    INTERRUPTED.load(Ordering::SeqCst)
}

pub fn get_terminal_size() -> (u16, u16) {
    let mut size = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    let result = unsafe {
        ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut size)
    };

    if result == 0 && size.ws_col > 0 && size.ws_row > 0 {
        (size.ws_col, size.ws_row)
    } else {
        (80, 24)
    }
}

pub fn get_terminal_width() -> u16 {
    get_terminal_size().0
}

pub fn get_terminal_height() -> u16 {
    get_terminal_size().1
}

pub fn clear_screen() {
    Command::new("clear")
        .status()
        .expect("failed to execute process");
}