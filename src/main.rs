use std::path::PathBuf;

use clap::Parser;
use directories::ProjectDirs;

use editor::Editor;

mod logger;
mod editor;
mod utils;
mod color_scheme;
mod line_buffer;
mod scroll_buffer;
mod status_line;


fn main() {
    std::panic::set_hook(Box::new(|panic_info| {
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            log::error!("Panic: {}", s);
        } else {
            log::error!("Panic occurred.");
        }
    }));

    match run_program() {
        Ok(_) => log::info!("Program existed successfully"),
        Err(e) => log::error!("Program failed: {}", e),
    }
}

fn run_program() -> std::io::Result<()> {
    logger::setup_logger()?;

    let args = Args::parse();

    let file_path = get_path_from_arg(args.filename.as_deref(), "contact.json");
    let config_path = get_path_from_arg(args.config.as_deref(), "config.toml");

    let mut editor = Editor::new(file_path, config_path, args.no_splash, args.sample_data)?;

    editor.init()?;

    editor.run()?;

    Ok(())
}

fn get_path_from_arg(arg: Option<&str>, filename: &str) -> PathBuf {
    arg
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            let proj_dirs = ProjectDirs::from("au", "popplestones", "RustyCrm").expect("Failed to get project directory");
            proj_dirs.config_dir().join(filename)
        })
}

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "Shane Poppleton")]
struct Args {
    #[clap(short, long)]
    filename: Option<String>,

    #[clap(short, long)]
    config: Option<String>,

    #[clap(long)]
    no_splash: bool,

    #[clap(long)]
    sample_data: bool,
}