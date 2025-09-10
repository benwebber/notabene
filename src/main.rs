use std::process::ExitCode;

#[cfg(feature = "cli")]
use notabene::cli;

#[cfg(feature = "cli")]
fn main() -> std::process::ExitCode {
    match cli::main() {
        Ok(_) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}
