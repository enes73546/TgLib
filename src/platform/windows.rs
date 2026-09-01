use windows::Win32::System::Console::{
    GetConsoleScreenBufferInfo,
    GetStdHandle,
    SetConsoleCtrlHandler,
    CONSOLE_SCREEN_BUFFER_INFO,
    STD_OUTPUT_HANDLE,
};

use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};

static INTERRUPTED: AtomicBool = AtomicBool::new(false);

unsafe extern "system" fn ctrl_c_handler(event: u32) -> windows::core::BOOL {
    if event == 0 {
        INTERRUPTED.store(true, Ordering::SeqCst);
        windows::core::BOOL(1)
    } else {
        windows::core::BOOL(0)
    }
}

pub fn install_ctrl_c_handler() {
    unsafe {
        SetConsoleCtrlHandler(Some(ctrl_c_handler), true)
            .expect("failed to install Ctrl+C handler");
    }
}

pub fn is_interrupted() -> bool {
    INTERRUPTED.load(Ordering::SeqCst)
}

pub fn clear_screen() {
    Command::new("cmd")
        .args(["/C", "cls"])
        .status()
        .expect("failed to execute process");
}

pub fn get_terminal_height() -> u16 {
    let mut csbi: CONSOLE_SCREEN_BUFFER_INFO = unsafe { std::mem::zeroed() };

    let handle = match unsafe { GetStdHandle(STD_OUTPUT_HANDLE) } {
        Ok(handle) => handle,
        Err(_) => return 24,
    };

    match unsafe { GetConsoleScreenBufferInfo(handle, &mut csbi) } {
        Ok(_) => (csbi.srWindow.Bottom - csbi.srWindow.Top + 1) as u16,
        Err(_) => 24,
    }
}

pub fn get_terminal_width() -> u16 {
    let mut csbi: CONSOLE_SCREEN_BUFFER_INFO = unsafe { std::mem::zeroed() };

    let handle = match unsafe { GetStdHandle(STD_OUTPUT_HANDLE) } {
        Ok(handle) => handle,
        Err(_) => return 80,
    };

    match unsafe { GetConsoleScreenBufferInfo(handle, &mut csbi) } {
        Ok(_) => (csbi.srWindow.Right - csbi.srWindow.Left + 1) as u16,
        Err(_) => 80,
    }
}