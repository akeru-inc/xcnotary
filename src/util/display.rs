use indicatif::{ProgressBar, ProgressStyle};

pub(crate) fn progress_bar(message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(120);
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⡘⠳⠈⠍⠉⡗⡙⠙⠚⡐⢋✔") //
            .template("{spinner:.white} {msg}"),
    );
    pb.set_message(message);
    pb
}
