use anyhow::Result;

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        fn main() -> Result<()> {
            idle_termquest::run()
        }
    } else {
        #[tokio::main]
        async fn main() -> Result<()> {
            idle_termquest::run().await
        }
    }
}
