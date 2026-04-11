mod apk;
mod apkpure;
mod firmware;
mod gui;
mod pem;
mod runner;
mod task;
mod tui;

fn main() -> anyhow::Result<()> {
    let use_tui = std::env::args().any(|a| a == "--tui");

    if use_tui {
        let rt = std::sync::Arc::new(tokio::runtime::Runtime::new()?);
        return tui::run(rt);
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Seestar Tool")
            .with_inner_size([760.0, 620.0])
            .with_min_inner_size([520.0, 440.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Seestar Tool",
        options,
        Box::new(|cc| Ok(Box::new(gui::SeestarApp::new(cc)))),
    )
    .map_err(|e| anyhow::anyhow!("{e}"))?;

    Ok(())
}
