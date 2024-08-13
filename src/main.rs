use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    query: String,

    #[arg(short, long)]
    file_path: String,
}

fn main() {
    let args = Args::parse();

    println!("minigrep");
    println!("{:?}", args);
}
