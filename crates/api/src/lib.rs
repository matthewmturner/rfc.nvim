use std::{collections::BTreeMap, collections::HashMap, collections::HashSet, io::Write};

// Exposed via an opaque pointer via FFI. If we weren't saving as Json we would probably be okay
// with `CString` but Json has stricter requirements for key values and `CString` when serialized does
// not meet them - so we use `String`.

/// A term in a document
pub type Term = String;
/// The source of the document
pub type Url = String;
/// Term frequency, tf(t,d), is the relative frequency of term t within document d.
pub type TermFreqs = HashMap<Term, f64>;
pub type DocTermFreqs = HashMap<Url, TermFreqs>;
pub type InvDocFreqs = HashMap<Term, f64>;
pub type DocsWithTerm = HashMap<Term, Vec<Url>>;
pub type TermScores = HashMap<Term, HashMap<Url, f64>>;
// pub type TermScores = HashMap<Term, Vec<(Url, f64)>>;

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
}

impl TfIdf {
    pub fn finish(&mut self) {
        // First, we collect all terms and the number of docs they appear in
        let mut term_counts: HashMap<&String, usize> = HashMap::new();
        let mut docs_with_term: HashMap<String, Vec<String>> = HashMap::new();
        for (doc, doc_terms) in &self.doc_tfs {
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
            let inv_fraction = (total_docs / docs_with_term) as f64;
            let scaled = inv_fraction.log10();
            self.idfs.insert(term.clone(), scaled);
        }

        // Then we compute the score for each term in all documents
        for (doc, doc_terms) in &self.doc_tfs {
            for (doc_term, freq) in doc_terms {
                if let Some(idf) = self.idfs.get(doc_term) {
                    let doc_term_score = freq * idf;
                    if let Some(ts) = self.term_scores.get_mut(doc_term) {
                        // ts.push((doc.clone(), doc_term_score));
                        ts.insert(doc.clone(), doc_term_score);
                    } else {
                        // let ts = vec![(doc.clone(), doc_term_score)];
                        let mut ts = HashMap::new();
                        ts.insert(doc.clone(), doc_term_score);
                        self.term_scores.insert(doc_term.clone(), ts);
                    }
                }
            }
        }
    }

    pub fn compute_search_scores(&self, search: String) {
        // Extract all the terms from the search
        let terms: Vec<&str> = search.split(" ").collect();

        let mut scores = Vec::new();
        for term in terms {
            if let Some(term_scores) = self.term_scores.get(term) {
                scores.push(term_scores.clone());
            }
        }

        let combined_scores = Self::combine_scores(scores);
    }

    pub fn combine_scores(scores: Vec<HashMap<String, f64>>) -> Vec<Url> {
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

        let mut scores_list: Vec<(Url, f64)> = combined_scores.into_iter().collect();

        // Sort by score in descending order
        scores_list.sort_by(|(_, a_score), (_, b_score)| b_score.partial_cmp(a_score).unwrap());

        // Return only the URLs
        scores_list.into_iter().map(|(url, _)| url).collect()
    }

    pub fn save(&self, path: &str) {
        let file = std::fs::File::create(path).unwrap();
        serde_json::to_writer(file, &self.term_scores).unwrap();
    }
}

pub fn save_json() -> std::io::Result<()> {
    let file = std::fs::File::create("val.json")?;
    serde_json::to_writer(file, &42).unwrap();
    Ok(())
}

pub fn save_input_number_as_json_to_custom_path(val: i32, path: &str) -> std::io::Result<()> {
    let file = std::fs::File::create(path)?;
    serde_json::to_writer(file, &val).unwrap();
    Ok(())
}

// pub fn save_tf_idf(tf_idf: Box<HashMap<String, HashMap<String, f64>>>, path: &str) {
//     let f = std::fs::File::create(path).unwrap();
//     eprintln!("tf_idf: {:?}", *tf_idf);
//     match serde_json::to_writer(f, &*tf_idf) {
//         Ok(_) => {}
//         Err(e) => {
//             let mut ef = std::fs::File::create("./error").unwrap();
//             ef.write_all(e.to_string().as_bytes()).unwrap();
//             ef.flush().unwrap();
//         }
//     }
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(1, 1);
    }
}
