#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // Claude Code invokes this as its statusLine; it must exit before any app start-up.
    if args.get(1).map(String::as_str) == Some(tokendaily_lib::CLAUDE_STATUSLINE_FLAG) {
        std::process::exit(tokendaily_lib::claude_statusline(&args));
    }
    tokendaily_lib::run();
}
