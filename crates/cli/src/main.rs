use std::path::PathBuf;

use clap::Parser;
use tf_idf::TermScores;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Args {
    #[arg(long, short)]
    index_path: PathBuf,
    #[arg(long, short)]
    term: Option<String>,
    #[arg(long, short)]
    search: Option<String>,
    #[arg(long, short)]
    keys: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!("Args: {args:?}");

    let file = std::fs::File::open(args.index_path)?;
    let start = std::time::Instant::now();
    let index: TermScores = simd_json::from_reader(file)?;
    println!("Reading index took {:?}", start.elapsed());

    if let Some(term) = args.term {
        if let Some(r) = index.get(&term) {
            println!("Results: {r:#?}");
        }
    } else if let Some(search) = args.search {
        tf_idf::compute_search_scores(search, index);
    } else if args.keys {
        println!("{:#?}", index.keys());
    }

    Ok(())
}
