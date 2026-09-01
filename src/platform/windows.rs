use windows::Win32::System::Console::{ GetConsoleScreenBufferInfo, GetStdHandle, CONSOLE_SCREEN_BUFFER_INFO, STD_OUTPUT_HANDLE, };
  use std::process::Command;

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
