use windows::UI::ViewManagement::{UIColorType, UISettings};

/// Wrap text with an ANSI 24-bit foreground color sequence.
/// `ESC[38;2;R;G;Bm` ... `ESC[0m`
fn fg(r: u8, g: u8, b: u8, text: &str) -> String {
    format!("\x1b[38;2;{r};{g};{b}m{text}\x1b[0m")
}

fn main() {
    // Make sure the console interprets ANSI escape sequences.
    #[cfg(windows)]
    enable_virtual_terminal();

    let settings = match UISettings::new() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to create UISettings: {e}");
            std::process::exit(1);
        }
    };

    // (name, description, UIColorType)
    let variants: [(&str, &str, UIColorType); 7] = [
        ("Accent",       "base accent color", UIColorType::Accent),
        ("AccentLight1", "light variant 1",   UIColorType::AccentLight1),
        ("AccentLight2", "light variant 2",   UIColorType::AccentLight2),
        ("AccentLight3", "light variant 3",   UIColorType::AccentLight3),
        ("AccentDark1",  "dark variant 1",    UIColorType::AccentDark1),
        ("AccentDark2",  "dark variant 2",    UIColorType::AccentDark2),
        ("AccentDark3",  "dark variant 3",    UIColorType::AccentDark3),
    ];

    let sep = "=".repeat(88);
    println!("{sep}");
    println!("Windows Accent Colors (Rust + WinRT)");
    println!("{sep}");
    println!(
        "{:<14} {:<10} {:<12} {:<18} {}",
        "Name", "HEX", "ARGB", "RGB", "Description"
    );
    println!("{}", "-".repeat(88));

    for (name, desc, kind) in variants {
        match settings.GetColorValue(kind) {
            Ok(color) => {
                let (a, r, g, b) = (color.A, color.R, color.G, color.B);
                let line = format!(
                    "{:<14} #{:02X}{:02X}{:02X}    #{:02X}{:02X}{:02X}{:02X}     ({:3},{:3},{:3})    {}",
                    name, r, g, b, a, r, g, b, r, g, b, desc
                );
                // Print the whole line using this accent color.
                println!("{}", fg(r, g, b, &line));
            }
            Err(e) => {
                println!("{:<14} <failed: {e}>", name);
            }
        }
    }

    println!("{sep}");
}

/// Enable `ENABLE_VIRTUAL_TERMINAL_PROCESSING` on the stdout console handle
/// so that ANSI escape sequences are rendered as colors.
#[cfg(windows)]
fn enable_virtual_terminal() {
    use windows::Win32::System::Console::{
        GetConsoleMode, GetStdHandle, SetConsoleMode, CONSOLE_MODE,
        ENABLE_VIRTUAL_TERMINAL_PROCESSING, STD_OUTPUT_HANDLE,
    };

    unsafe {
        if let Ok(handle) = GetStdHandle(STD_OUTPUT_HANDLE) {
            let mut mode = CONSOLE_MODE(0);
            if GetConsoleMode(handle, &mut mode).is_ok() {
                let _ = SetConsoleMode(handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
            }
        }
    }
}
