use std::{fs::File, path::PathBuf, time::Instant};

use clap::{Parser, Subcommand};
use rfsee_tf_idf::{
    error::{RFSeeError, RFSeeResult},
    get_index_path, search_index, Index, TfIdf,
};

#[derive(Clone, Debug, Parser)]
#[command(version, about)]
pub struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Clone, Debug, Subcommand)]
enum Command {
    Index {
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    Search {
        #[arg(short, long)]
        terms: String,
        #[arg(short, long)]
        index_path: Option<PathBuf>,
    },
}

extern "C" fn fetch_progress_cb(progress: f64) {
    println!("Fetching RFCs progress: {progress:.2}%")
}

extern "C" fn parse_progress_cb(progress: f64) {
    println!("Parsing RFCs progress: {progress:.2}%")
}

fn handle_command(args: Args) -> RFSeeResult<()> {
    if let Some(command) = args.clone().command {
        match command {
            Command::Index { path } => {
                println!("Indexing RFCs");
                let start = Instant::now();
                let mut index = TfIdf::default();
                index.par_load_rfcs(fetch_progress_cb, parse_progress_cb)?;
                println!("Loading RFCs took {:?}", start.elapsed());
                let building_index_start = Instant::now();
                index.finish();
                println!("Building index took {:?}", building_index_start.elapsed());
                let saving_start = Instant::now();
                let index_path = get_index_path(path)?;
                index.save(&index_path);
                println!("Saving index took {:?}", saving_start.elapsed());
            }
            Command::Search { terms, index_path } => {
                let start = Instant::now();
                let index_path = get_index_path(index_path)?;
                let file =
                    File::open(index_path).map_err(|e| RFSeeError::IOError(e.to_string()))?;
                let index: Index = simd_json::from_reader(file)
                    .map_err(|e| RFSeeError::ParseError(e.to_string()))?;
                println!("Reading index file took: {:?}", start.elapsed());
                let results = search_index(terms, index);
                println!("Total search time: {:?}", start.elapsed());
                println!("Docs: {results:#?}");
            }
        }
    }
    Ok(())
}

fn main() -> RFSeeResult<()> {
    let args = Args::parse();
    handle_command(args)?;
    Ok(())
}
