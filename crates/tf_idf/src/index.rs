use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::{fetch::fetch, threadpool};
use regex::Regex;
use serde::{Deserialize, Serialize};

/// A term in a document
pub type Term = String;
/// The url of the source document
pub type Url = String;
/// Term frequency, tf(t,d), is the relative frequency of term t within document d.  Keys are the
/// terms and values are the frequency of the term
pub type TermFreqs = HashMap<Term, f32>;
/// Mapping from a Url to the term frequencies for the document at that url.
pub type DocTermFreqs = HashMap<Url, TermFreqs>;
/// For each term its inverse document frequency which is computed as:
///
/// log(total_docs / (docs_with_term + ε))
///
/// ε used so that terms which are in all documents, such as "HTTP", can still have their term
/// frequency apply.  Without this the final score would be 0 as the IDF would be 0.
pub type InvDocFreqs = HashMap<Term, f32>;
/// Map of terms to a vector of documents that have that term
pub type DocsWithTerm = HashMap<Term, Vec<Url>>;
pub type ProcessedRfcs = HashMap<Url, ProcessedRfc>;
pub type RfcNumber = i32;
pub type TermScore = i32;
pub type RfcDetailsMap = HashMap<RfcNumber, RfcDetails>;

const RFC_INDEX_URL: &str = "https://www.ietf.org/rfc/rfc-index.txt";
const RFC_DELIMITER: &str = "\n\n";
const RFC_EDITOR_URL_BASE: &str = "https://www.rfc-editor.org/rfc/rfc";
const RFC_EDITOR_FILE_TYPE: &str = "txt";

const WORD_MATCH_REGEX: &str = r"(\w+)";
/// We have an epsilon value to account for some terms, like "HTTP", being in all RFCs.
const EPSILON: f32 = 0.0001;
/// How to split the user provided search
const SEARCH_TERMS_DELIMITER: &str = " ";

const INDEX_FILE_NAME: &str = "index.json";
const DEFAULT_INDEX_PATH: &str = "/tmp/index.json";

#[derive(Debug)]
pub struct RfcEntry {
    pub number: i32,
    pub url: String,
    pub title: String,
    pub content: Option<String>,
}

pub struct ProcessedRfc {
    number: i32,
    term_freqs: TermFreqs,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RfcDetails {
    title: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Index {
    rfc_details: RfcDetailsMap,
    /// Map of terms to map of docs with that term and its score
    term_scores: HashMap<Term, HashMap<RfcNumber, TermScore>>,
}

#[repr(C)]
#[derive(Debug)]
pub struct RfcSearchResult {
    pub url: String,
    pub title: String,
}

pub fn get_index_path(custom_path: Option<PathBuf>) -> PathBuf {
    if let Some(path) = custom_path {
        path
    } else if let Some(project_dirs) =
        directories::ProjectDirs::from("com", "matthewmturner", "rfsee")
    {
        let data_dir = project_dirs.data_dir();
        if !data_dir.exists() {
            std::fs::create_dir_all(data_dir).unwrap();
        }
        data_dir.to_path_buf().join(INDEX_FILE_NAME)
    } else {
        PathBuf::from(DEFAULT_INDEX_PATH)
    }
}

/// Return the raw `String` contents of IETF RFC index
fn fetch_rfc_index() -> anyhow::Result<String> {
    let rfc_index_content = fetch(RFC_INDEX_URL)?;
    Ok(rfc_index_content)
}

/// Parse raw `String` contents of RFC index and return `Vec` of `&str` for each item after
/// splitting on `RFC_DELIMITER`
fn parse_rfc_index(content: &str) -> anyhow::Result<Vec<&str>> {
    let found = content.find("0001");
    match found {
        Some(idx) => {
            let raw_rfcs = &content[idx..];
            let splitted = raw_rfcs.split(RFC_DELIMITER).collect();
            Ok(splitted)
        }
        None => anyhow::bail!("Unable to parse RFC index"),
    }
}

/// Parse raw RFC `String` contents into the RFC number and its title
fn parse_rfc(rfc_content: &str) -> anyhow::Result<(i32, &str)> {
    if let Some((rfc_num, title)) = rfc_content.split_once(" ") {
        let parsed_num: i32 = rfc_num.parse()?;
        Ok((parsed_num, title))
    } else {
        anyhow::bail!("Unable to parse RFC number {rfc_content}");
    }
}

pub fn fetch_rfcs() -> anyhow::Result<Vec<RfcEntry>> {
    let rfc_index_content = fetch(RFC_INDEX_URL)?;
    let rfcs = parse_rfcs_index(rfc_index_content)?;
    Ok(rfcs)
}

fn fetch_rfc(raw_rfc: &str) -> anyhow::Result<RfcEntry> {
    if let Ok((rfc_num, title)) = parse_rfc(raw_rfc) {
        let url = format!("{RFC_EDITOR_URL_BASE}{rfc_num}.txt");
        if let Ok(content) = fetch(&url) {
            Ok(RfcEntry {
                number: rfc_num,
                url: url.clone(),
                title: title.replace("\n     ", " ").to_string(),
                content: Some(content),
            })
        } else {
            anyhow::bail!("Unable to fetch RFC")
        }
    } else {
        anyhow::bail!("Unable to parse raw RFC")
    }
}

pub fn parse_rfcs_index(content: String) -> anyhow::Result<Vec<RfcEntry>> {
    let found = content.find("0001");
    match found {
        Some(idx) => {
            let mut rfcs = Vec::new();
            let raw_rfcs = &content[idx..];
            let splitted = raw_rfcs.split(RFC_DELIMITER);
            for raw_rfc in splitted {
                if let Some((rfc_num, title)) = raw_rfc.split_once(" ") {
                    let parsed_num: i32 = rfc_num.parse()?;
                    if parsed_num % 1000 == 0 {
                        println!("Fetching RFC number {parsed_num}");
                    }
                    let url = format!("https://www.rfc-editor.org/rfc/rfc{parsed_num}.txt");
                    let content = match fetch(&url) {
                        Ok(content) => Some(content),
                        Err(_) => None,
                    };
                    rfcs.push(RfcEntry {
                        number: parsed_num,
                        url: url.clone(),
                        title: title.replace("\n     ", " ").to_string(),
                        content,
                    })
                }
            }
            Ok(rfcs)
        }
        None => anyhow::bail!("Invalid RFC index conetent"),
    }
}

#[repr(C)]
#[derive(Default)]
pub struct TfIdf {
    /// Term frequencies for the document at a url.
    pub doc_tfs: DocTermFreqs,
    /// The inverse document frequency is a measure of how much information the word provides, i.e.,
    /// how common or rare it is across all documents. It is the logarithmically scaled inverse fraction
    /// of the documents that contain the word (obtained by dividing the total number of documents by
    /// the number of documents containing the term, and then taking the logarithm of that quotient).
    pub idfs: InvDocFreqs,
    /// Map of terms to a list of documents that contain that term
    pub docs_with_term: DocsWithTerm,
    pub processed_rfcs: ProcessedRfcs,
    pub index: Index,
}

impl TfIdf {
    pub fn load_rfcs(&mut self) -> anyhow::Result<()> {
        let raw_rfc_index = fetch_rfc_index()?;
        let raw_rfcs = parse_rfc_index(&raw_rfc_index)?;
        for (i, raw_rfc) in raw_rfcs.into_iter().enumerate() {
            if i % 1000 == 0 {
                eprintln!("Processing RFC #{i}");
            }
            let (rfc_num, title) = parse_rfc(raw_rfc)?;
            let url = format!("{RFC_EDITOR_URL_BASE}{rfc_num}.txt");
            let maybe_parsed_rfc = match fetch(&url) {
                Ok(content) => Some(RfcEntry {
                    number: rfc_num,
                    url: url.clone(),
                    title: title.replace("\n     ", " ").to_string(),
                    content: Some(content),
                }),
                Err(_) => None,
            };
            if let Some(parsed_rfc) = maybe_parsed_rfc {
                if parsed_rfc.content.is_some() {
                    self.add_rfc_entry(parsed_rfc)
                }
            }
        }

        Ok(())
    }

    /// Load the RFCs in parallel using a threadpool
    pub fn par_load_rfcs(&mut self) -> anyhow::Result<()> {
        let pool = threadpool::ThreadPool::new(12);
        let raw_rfc_index = fetch_rfc_index()?;
        let raw_rfcs = parse_rfc_index(&raw_rfc_index)?;

        let parsed_rfcs: Vec<RfcEntry> = Vec::new();
        let parsed_rfcs = Arc::new(Mutex::new(parsed_rfcs));

        let remaining = raw_rfcs.len();
        let remaining = Arc::new(Mutex::new(remaining));

        for rfc in raw_rfcs.into_iter() {
            let string = rfc.to_string();
            let remaining = Arc::clone(&remaining);
            let parsed_rfcs = Arc::clone(&parsed_rfcs);
            pool.execute(move || {
                if let Ok(r) = fetch_rfc(&string) {
                    let mut guard = parsed_rfcs.lock().unwrap();
                    guard.push(r);
                };
                let mut guard = remaining.lock().unwrap();
                *guard -= 1;
            })
        }

        let mut finished = false;
        while !finished {
            let remaining = remaining.clone();
            let guard = remaining.lock().unwrap();
            if *guard == 0 {
                finished = true
            } else {
                drop(guard);
                std::thread::sleep(Duration::from_secs(5));
            }
        }

        match Arc::try_unwrap(parsed_rfcs) {
            Ok(mutex) => match mutex.into_inner() {
                Ok(rfcs) => {
                    for rfc in rfcs {
                        self.add_rfc_entry(rfc);
                    }
                }
                Err(err) => anyhow::bail!(err),
            },
            Err(_) => {
                anyhow::bail!("More than one reference remaining")
            }
        }

        Ok(())
    }

    /// Process `RfcEntry` by computing it's term frequencies and add it to index
    pub fn add_rfc_entry(&mut self, rfc: RfcEntry) {
        let re = Regex::new(WORD_MATCH_REGEX).unwrap();

        if let Some(content) = &rfc.content {
            let mut term_counts: HashMap<&str, usize> = HashMap::new();
            let mut tfs = TermFreqs::new();
            let mut terms = 0;

            for found in re.find_iter(content) {
                if let Some(k) = term_counts.get_mut(found.as_str()) {
                    *k += 1
                } else {
                    term_counts.insert(found.as_str(), 1);
                }
                terms += 1
            }

            for (t, c) in term_counts {
                let frequency = c as f32 / terms as f32;
                tfs.insert(t.to_string(), frequency);
            }

            let indexed_rfc = ProcessedRfc {
                number: rfc.number,
                term_freqs: tfs,
            };

            self.index
                .rfc_details
                .insert(rfc.number, RfcDetails { title: rfc.title });

            self.processed_rfcs.insert(rfc.url, indexed_rfc);
        }
    }

    /// Take all the processed documents and their term frequencies to compute the final term
    /// scores
    pub fn finish(&mut self) {
        // First, we collect all terms and the number of docs they appear in
        let mut term_counts: HashMap<&String, usize> = HashMap::new();
        for indexed_rfc in self.processed_rfcs.values() {
            for term in indexed_rfc.term_freqs.keys() {
                if let Some(v) = term_counts.get(term) {
                    term_counts.insert(term, v + 1);
                } else {
                    term_counts.insert(term, 1);
                }
            }
        }

        // Then we compute the inverse document frequency for each term
        let total_docs = self.processed_rfcs.len();
        for (term, docs_with_term) in term_counts {
            let inv_fraction = (total_docs as f32) / ((docs_with_term as f32) + EPSILON);
            let scaled = inv_fraction.log10();
            self.idfs.insert(term.clone(), scaled);
        }

        // Then we compute the score for each term in all documents
        self.processed_rfcs.iter().for_each(|(_doc, rfc)| {
            for (doc_term, freq) in &rfc.term_freqs {
                if let Some(idf) = self.idfs.get(doc_term) {
                    // there are often lots of 0s preceding actual score, we can remove those
                    let doc_term_score = (freq * idf) * 1_000_000_000.0;
                    let rounded_doc_term_score = doc_term_score.round() as i32;
                    // let rounded_doc_term_score = (doc_term_score * 100000.0).round() / 100000.0;
                    if let Some(term_scores_per_doc) = self.index.term_scores.get_mut(doc_term) {
                        term_scores_per_doc.insert(rfc.number, rounded_doc_term_score);
                    } else {
                        let mut term_scores_per_doc = HashMap::new();
                        term_scores_per_doc.insert(rfc.number, rounded_doc_term_score);
                        self.index
                            .term_scores
                            .insert(doc_term.to_string(), term_scores_per_doc);
                    }
                }
            }
        });
    }

    pub fn save(&self, path: &Path) {
        {
            let index_file = std::fs::File::create(path).unwrap();
            simd_json::to_writer(index_file, &self.index).unwrap();
        }
    }
}

pub fn combine_scores(scores: Vec<HashMap<i32, i32>>) -> Vec<i32> {
    let mut combined_scores: HashMap<i32, i32> = HashMap::new();
    for score in scores {
        for (rfc_num, term_score) in score {
            if let Some(combined_doc_score) = combined_scores.get_mut(&rfc_num) {
                *combined_doc_score += term_score;
            } else {
                combined_scores.insert(rfc_num, term_score);
            }
        }
    }

    let mut scores_list: Vec<(i32, i32)> = combined_scores.into_iter().collect();

    // Sort by score in descending order
    scores_list.sort_by(|(_, a_score), (_, b_score)| b_score.partial_cmp(a_score).unwrap());

    // Return only the URLs
    scores_list.into_iter().map(|(rfc, _)| rfc).collect()
}

pub fn compute_search_scores(search: String, index: Index) -> Vec<RfcSearchResult> {
    // Extract all the terms from the search
    let terms: Vec<&str> = search.split(SEARCH_TERMS_DELIMITER).collect();

    // Extract the top documents for each term
    let mut scores = Vec::new();
    for term in terms {
        if let Some(term_scores) = index.term_scores.get(term) {
            scores.push(term_scores.clone());
        }
    }

    // Combine the scores by adding them for each document
    let rfcs = combine_scores(scores);
    rfcs.iter()
        .map(|n| {
            if let Some(details) = index.rfc_details.get(n) {
                RfcSearchResult {
                    url: format!("{RFC_EDITOR_URL_BASE}{n}.{RFC_EDITOR_FILE_TYPE}"),
                    title: details.title.clone(),
                }
            } else {
                RfcSearchResult {
                    url: format!("{RFC_EDITOR_URL_BASE}{n}.{RFC_EDITOR_FILE_TYPE}"),
                    title: "MISSING TITLE".to_string(),
                }
            }
        })
        .collect()
}
