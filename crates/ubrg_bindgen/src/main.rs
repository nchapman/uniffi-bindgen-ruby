use clap::Parser;

fn main() -> anyhow::Result<()> {
    let cli = ubrg_bindgen::cli::Cli::parse();
    ubrg_bindgen::run(cli)
}
