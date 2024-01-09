use anyhow::Result;
use context::Config;
use ratatui::layout::Rect;

mod core;
use core::App;

mod event;
pub use event::Event;

mod context;
pub use context::Context;

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        pub async fn run() -> Result<()> {
            Ok(())
        }
    } else if #[cfg(target_os = "windows")] {
        pub async fn run() -> Result<()> {
            Ok(())
        }
    } else  {
        mod backends;
        use backends::Crossterm;

        pub async fn run() -> Result<()> {
            let mut app = App::new();

            let mut ctx = Context::default()
                .config(Config {
                    tick_rate: 7.,
                    frame_rate: 7.
                });

            let mut backend = Crossterm::new()?
                .tick_rate(ctx.config.tick_rate)
                .frame_rate(ctx.config.frame_rate)
                .mouse(true)
                .paste(false);

            backend.enter()?;

            ctx.push_log("Init\n".to_owned());

            loop {
                if let Some(event) = backend.next().await {
                    match event {
                        Event::Init => app.init()?,
                        Event::Render => { backend.draw(|frame| app.render(frame, &ctx))?; },
                        Event::Resize(w, h) => {
                            backend.terminal.resize(Rect::new(0, 0, w, h))?;
                            backend.draw(|frame| app.render(frame, &ctx))?;
                        },
                        Event::Tick => app.update(&mut ctx),
                        event => app.handle_event(&mut ctx, event)
                    }
                }
                if ctx.should_quit {
                    backend.stop()?;
                    break;
                }
            }
            backend.exit()?;
            Ok(())
        }
    }
}
