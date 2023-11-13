use anyhow::Result;
use clap::Parser;
use std::fs::read_dir;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    selected_path: String,

    #[arg(short, long)]
    all_path: String,

    #[arg(short, long)]
    output_path: String,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let file_names = read_dir(args.selected_path)?
        .map(|res| res.unwrap().file_name())
        .collect::<Vec<_>>();

    let raw_file_names = file_names
        .into_iter()
        .map(|file_name| file_name.into_string().unwrap().replace(".JPG", ".RAF"))
        .collect::<Vec<String>>();

    let dir_path = std::path::Path::new(&args.output_path);

    if !dir_path.exists() {
        std::fs::create_dir_all(dir_path)?;
    }

    raw_file_names.iter().for_each(|raw_file_name| {
        let raw_file_path = format!("{}/{}", args.all_path, raw_file_name);
        let output_file_path = format!("{}/{}", args.output_path, raw_file_name);
        println!("copying {} to {}", raw_file_path, output_file_path);
        std::fs::copy(raw_file_path, output_file_path).unwrap();
    });

    Ok(())
}
