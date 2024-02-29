//pub mod h5parm;

use clap::Parser;

extern crate lofar_h5parm_rs;

/// A Rust port of the polconv functionality of h5_merger.py by Jurjen de Jong.
#[derive(Parser, Debug)]
#[command(name = "lofar-H5info")]
#[command(author = "Frits Sweijen")]
#[command(version = "0.0.0")]
#[command(
    help_template = "{name} \nVersion: {version} \nAuthor: {author}\n{about-section} \n {usage-heading} {usage} \n {all-args} {tab}"
)]
// #[clap(author="Author Name", version, about="")]
struct Args {
    /// H5parm to summarise.
    #[arg(long)]
    h5parm: String,
    /// SolSet to display.
    #[arg(long, default_value = "")]
    solset: String,
}

fn summarise_h5parm(h5parm: &String) {
    let h5name = h5parm.split("/").last().unwrap();
    println!("Summarising {}\n", h5name);
    let h5 = lofar_h5parm_rs::H5parm::open(h5parm, false).expect("Failed to read H5parm.");
    println!("Solution sets:");
    for ss in h5.solsets {
        println!("|-{}", ss.name);
        for st in ss.soltabs {
            println!("|---{:<15} {}", st.name, st.get_polarisations());
        }
        println!();
    }
}

fn main() {
    let args = Args::parse();
    println!("H5parm: {}\n", args.h5parm);
    summarise_h5parm(&args.h5parm);
}
