use app::App;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use shell::{detect_shell, Shell};

mod app;
mod shell;

fn main() {
    let shell: Shell = detect_shell();
    let commands = shell.get_commands().unwrap();
    let mut app = App::new(commands);
    match app.run() {
        Ok(Some(selected_command)) => {
            println!("Copied command: {}", selected_command);
            if let Ok(mut ctx) = ClipboardContext::new() {
                let _ = ctx.set_contents(selected_command);
            }
        }
        Ok(None) => println!("No command selected"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
