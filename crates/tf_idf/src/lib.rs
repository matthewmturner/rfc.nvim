use std::collections::HashMap;

use fetch::fetch;
use regex::Regex;
use serde::{Deserialize, Serialize};

/// Exposed via an opaque pointer via FFI. If we weren't saving as Json we would probably be okay
/// with `CString` but Json has stricter requirements for key values and `CString` when serialized does
/// not meet them - so we use `String`.

/// A term in a document
pub type Term = String;
/// The source of the document
pub type Url = String;
/// Term frequency, tf(t,d), is the relative frequency of term t within document d.
pub type TermFreqs = HashMap<Term, f32>;
pub type DocTermFreqs = HashMap<Url, TermFreqs>;
pub type InvDocFreqs = HashMap<Term, f32>;
pub type DocsWithTerm = HashMap<Term, Vec<Url>>;
pub type TermScores = HashMap<Term, HashMap<Url, f32>>;
pub type ProcessedRfcs = HashMap<Url, ProcessedRfc>;
pub type TermScoresV2 = HashMap<Term, HashMap<Url, SerializedRfcWithTermScore>>;
pub type RfcNumber = i32;
pub type TermScore = f32;
pub type RfcDetailsMap = HashMap<RfcNumber, RfcDetails>;

const RFC_INDEX_URL: &str = "https://www.ietf.org/rfc/rfc-index.txt";
const RFC_DELIMITER: &str = "\n\n";
// const RFC_EDITOR_ADDR: &str = "www.rfc-editor.org:443";
// const RFC_EDITOR_DOMAIN: &str = "www.rfc-editor.org";

const WORD_MATCH_REGEX: &str = r"(\w+)";
/// We have an epsilon value to account for some terms, like "HTTP", being in all RFCs.
const EPSILON: f32 = 0.0001;
const SEARCH_TERMS_DELIMITER: &str = " ";

pub struct RfcEntry {
    pub number: i32,
    pub url: String,
    pub title: String,
    pub content: Option<String>,
}

// pub struct IndexedRfc {
//     number: i32,
//     url: String,
//     title: String,
//     term_freqs: TermFreqs,
// }

pub struct ProcessedRfc {
    number: i32,
    url: String,
    term_freqs: TermFreqs,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RfcWithTermScore {
    number: i32,
    url: String,
    title: String,
    score: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RfcDetails {
    title: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Index {
    rfc_details: RfcDetailsMap,
    term_scores: HashMap<Term, HashMap<RfcNumber, TermScore>>,
}

/// Excludes URL because it can be constructed from number
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SerializedRfcWithTermScore {
    number: i32,
    title: String,
    score: f32,
}

pub fn fetch_rfcs() -> anyhow::Result<Vec<RfcEntry>> {
    let rfc_index_content = fetch(RFC_INDEX_URL)?;
    let rfcs = parse_rfcs_index(rfc_index_content)?;
    Ok(rfcs)
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
                        title: title.to_string(),
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
    /// Map of terms to map of docs with that term and its score
    pub term_scores: TermScores,
    pub processed_rfcs: ProcessedRfcs,
    pub term_scores_v2: TermScoresV2,
    pub index: Index,
}

impl TfIdf {
    pub fn add_doc(&mut self, url: &str, doc: &str) {
        let re = Regex::new(WORD_MATCH_REGEX).unwrap();

        let mut term_counts: HashMap<&str, usize> = HashMap::new();
        let mut tfs = TermFreqs::new();
        let mut terms = 0;

        for found in re.find_iter(doc) {
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

        self.doc_tfs.insert(url.to_string(), tfs);
    }

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
                url: rfc.url.clone(),
                term_freqs: tfs,
            };

            self.index
                .rfc_details
                .insert(rfc.number, RfcDetails { title: rfc.title });

            self.processed_rfcs.insert(rfc.url, indexed_rfc);
        }
    }

    pub fn finish(&mut self) {
        // First, we collect all terms and the number of docs they appear in
        let mut term_counts: HashMap<&String, usize> = HashMap::new();
        for doc_terms in self.doc_tfs.values() {
            for term in doc_terms.keys() {
                if let Some(v) = term_counts.get(term) {
                    term_counts.insert(term, v + 1);
                } else {
                    term_counts.insert(term, 1);
                }
            }
        }

        // Then we compute the inverse document frequency for each term
        let total_docs = self.doc_tfs.len();
        for (term, docs_with_term) in term_counts {
            let inv_fraction = (total_docs as f32) / ((docs_with_term as f32) + EPSILON);
            let scaled = inv_fraction.log10();
            self.idfs.insert(term.clone(), scaled);
        }

        // Then we compute the score for each term in all documents
        for (doc, doc_terms) in &self.doc_tfs {
            for (doc_term, freq) in doc_terms {
                if let Some(idf) = self.idfs.get(doc_term) {
                    let doc_term_score = freq * idf;
                    if let Some(ts) = self.term_scores.get_mut(doc_term) {
                        ts.insert(doc.clone(), doc_term_score);
                    } else {
                        let mut ts = HashMap::new();
                        ts.insert(doc.clone(), doc_term_score);
                        self.term_scores.insert(doc_term.clone(), ts);
                    }
                }
            }
        }
    }

    pub fn finish_v2(&mut self) {
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
        for (_doc, rfc) in &self.processed_rfcs {
            for (doc_term, freq) in &rfc.term_freqs {
                if let Some(idf) = self.idfs.get(doc_term) {
                    let doc_term_score = freq * idf;
                    if let Some(term_scores_per_doc) = self.index.term_scores.get_mut(doc_term) {
                        term_scores_per_doc.insert(rfc.number, doc_term_score);
                    } else {
                        let mut term_scores_per_doc = HashMap::new();
                        term_scores_per_doc.insert(rfc.number, doc_term_score);
                        self.index
                            .term_scores
                            .insert(doc_term.to_string(), term_scores_per_doc);
                    }
                }
            }
        }
    }

    pub fn compute_search_scores(&self, search: String) -> Vec<String> {
        // Extract all the terms from the search
        let terms: Vec<&str> = search.split(" ").collect();

        let mut scores = Vec::new();
        for term in terms {
            if let Some(term_scores) = self.term_scores.get(term) {
                scores.push(term_scores.clone());
            }
        }

        Self::combine_scores(scores)
    }

    pub fn combine_scores(scores: Vec<HashMap<String, f32>>) -> Vec<Url> {
        let mut combined_scores = HashMap::new();
        for score in scores {
            for (url, doc_score) in score {
                if let Some(combined_doc_score) = combined_scores.get_mut(&url) {
                    *combined_doc_score += doc_score;
                } else {
                    combined_scores.insert(url.clone(), doc_score);
                }
            }
        }

        let mut scores_list: Vec<(Url, f32)> = combined_scores.into_iter().collect();

        // Sort by score in descending order
        scores_list.sort_by(|(_, a_score), (_, b_score)| b_score.partial_cmp(a_score).unwrap());

        // Return only the URLs
        scores_list.into_iter().map(|(url, _)| url).collect()
    }

    pub fn save(&self, _path: &str) {
        let index_file = std::fs::File::create("/tmp/index.json").unwrap();
        // let idfs_file = std::fs::File::create("/tmp/idfs.json").unwrap();
        // let doc_tfs_file = std::fs::File::create("/tmp/doc_tfs.json").unwrap();
        simd_json::to_writer(index_file, &self.index).unwrap();
        // simd_json::to_writer(idfs_file, &self.idfs).unwrap();
        // simd_json::to_writer(doc_tfs_file, &self.doc_tfs).unwrap();
    }
}

pub fn combine_scores(scores: Vec<HashMap<String, f32>>) -> Vec<Url> {
    let mut combined_scores = HashMap::new();
    for score in scores {
        for (url, doc_score) in score {
            if let Some(combined_doc_score) = combined_scores.get_mut(&url) {
                *combined_doc_score += doc_score;
            } else {
                combined_scores.insert(url.clone(), doc_score);
            }
        }
    }

    let mut scores_list: Vec<(Url, f32)> = combined_scores.into_iter().collect();

    // Sort by score in descending order
    scores_list.sort_by(|(_, a_score), (_, b_score)| b_score.partial_cmp(a_score).unwrap());

    // Return only the URLs
    scores_list.into_iter().map(|(url, _)| url).collect()
}

pub fn compute_search_scores(search: String, term_scores: &TermScores) -> Vec<String> {
    // Extract all the terms from the search
    let terms: Vec<&str> = search.split(SEARCH_TERMS_DELIMITER).collect();

    // Extract the top documents for each term
    let mut scores = Vec::new();
    for term in terms {
        if let Some(term_scores) = term_scores.get(term) {
            scores.push(term_scores.clone());
        }
    }

    // Combine the scores by adding them for each document
    combine_scores(scores)
}

pub fn combine_scores_v2(
    scores: Vec<HashMap<String, SerializedRfcWithTermScore>>,
) -> Vec<SerializedRfcWithTermScore> {
    let mut combined_scores: HashMap<String, SerializedRfcWithTermScore> = HashMap::new();
    for score in scores {
        for (url, rfc) in score {
            if let Some(combined_doc_score) = combined_scores.get_mut(&url) {
                combined_doc_score.score += rfc.score;
            } else {
                combined_scores.insert(url, rfc);
            }
        }
    }

    let mut scores_list: Vec<(Url, SerializedRfcWithTermScore)> =
        combined_scores.into_iter().collect();

    // Sort by score in descending order
    scores_list
        .sort_by(|(_, a_score), (_, b_score)| b_score.score.partial_cmp(&a_score.score).unwrap());

    // Return only the URLs
    scores_list.into_iter().map(|(_, rfc)| rfc).collect()
}

pub fn compute_search_scores_v2(
    search: String,
    term_scores: &TermScoresV2,
) -> Vec<SerializedRfcWithTermScore> {
    // Extract all the terms from the search
    let terms: Vec<&str> = search.split(SEARCH_TERMS_DELIMITER).collect();

    // Extract the top documents for each term
    let mut scores = Vec::new();
    for term in terms {
        if let Some(term_scores) = term_scores.get(term) {
            scores.push(term_scores.clone());
        }
    }

    // Combine the scores by adding them for each document
    combine_scores_v2(scores)
}

pub fn combine_scores_v3(scores: Vec<HashMap<i32, f32>>) -> Vec<i32> {
    let mut combined_scores: HashMap<i32, f32> = HashMap::new();
    for score in scores {
        for (rfc_num, term_score) in score {
            if let Some(combined_doc_score) = combined_scores.get_mut(&rfc_num) {
                *combined_doc_score += term_score;
            } else {
                combined_scores.insert(rfc_num, term_score);
            }
        }
    }

    let mut scores_list: Vec<(i32, f32)> = combined_scores.into_iter().collect();

    // Sort by score in descending order
    scores_list.sort_by(|(_, a_score), (_, b_score)| b_score.partial_cmp(&a_score).unwrap());

    // Return only the URLs
    scores_list.into_iter().map(|(rfc, _)| rfc).collect()
}

#[derive(Debug)]
pub struct RfcSearchResult {
    url: String,
    title: String,
}

pub fn compute_search_scores_v3(search: String, index: Index) -> Vec<RfcSearchResult> {
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
    let rfcs = combine_scores_v3(scores);
    rfcs.iter()
        .map(|n| {
            if let Some(details) = index.rfc_details.get(n) {
                RfcSearchResult {
                    url: format!("https://www.rfc-editor.org/rfc/rfc{n}.txt"),
                    title: details.title.clone(),
                }
            } else {
                RfcSearchResult {
                    url: format!("https://www.rfc-editor.org/rfc/rfc{n}.txt"),
                    title: "MISSING TITLE".to_string(),
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(1, 1);
    }
}
