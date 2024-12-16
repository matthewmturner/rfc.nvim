use std::{fs::File, path::PathBuf, time::Instant};

use clap::{Parser, Subcommand};
use tf_idf::{compute_search_scores, fetch_rfcs, Index, TfIdf};

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    Index {
        #[arg(short, long)]
        path: PathBuf,
    },
    Search {
        #[arg(short, long)]
        terms: String,
        #[arg(short, long, default_value_t = String::from("/tmp/index.json"))]
        index_path: String,
    },
}

fn handle_command(args: Args) -> anyhow::Result<()> {
    if let Some(command) = args.command {
        match command {
            Command::Index { path } => {
                println!("Indexing RFCs");
                let start = Instant::now();
                let rfcs = fetch_rfcs()?;
                println!("Fetching RFCs took {:?}", start.elapsed());
                let processing_start = Instant::now();
                let mut index = TfIdf::default();
                for rfc in rfcs {
                    if rfc.number % 1000 == 0 {
                        println!("Processing RFC {}", rfc.number);
                    }
                    if rfc.content.is_some() {
                        index.add_rfc_entry(rfc);
                    }
                }
                println!("Processing RFCs took {:?}", processing_start.elapsed());
                let building_index_start = Instant::now();
                index.finish();
                println!("Building index took {:?}", building_index_start.elapsed());
                let saving_start = Instant::now();
                index.save(path.to_str().unwrap_or("/tmp/index.json"));
                println!("Saving index took {:?}", saving_start.elapsed());
            }
            Command::Search { terms, index_path } => {
                let start = Instant::now();
                let file = File::open(index_path)?;
                let index: Index = simd_json::from_reader(file)?;
                println!("Opening index file took: {:?}", start.elapsed());
                let results = compute_search_scores(terms, index);
                println!("Total search time: {:?}", start.elapsed());
                println!("Docs: {results:#?}");
            }
        }
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    handle_command(args)?;
    Ok(())
}
