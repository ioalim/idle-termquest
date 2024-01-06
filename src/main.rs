use anyhow::Result;

fn main() -> Result<()> {
    if cfg!(target_os = "windows") {
        idle_termquest::run_windows()
    } else if cfg!(target_arch = "wasm32") {
        idle_termquest::run_wasm()
    } else {
        idle_termquest::run()
    }
}
