use anyhow::Result;

cfg_if::cfg_if! {
    // i don't event know if this whould work, but i will leave it like this in the mean time
    if #[cfg(target_arch = "wasm32")] {
        #[tokio::main(flavor = "current_thread")]
        async fn main() -> Result<()> {
            idle_termquest::run().await
        }
    } else {
        #[tokio::main]
        async fn main() -> Result<()> {
            idle_termquest::run().await
        }
    }
}
