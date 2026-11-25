//! Fuzzy matching for the command palette: the query's characters must
//! appear in order in the candidate (case-insensitive). Consecutive
//! characters and word starts score higher; gaps cost a little.

/// Score of `candidate` for `query`, or `None` if it does not match.
/// An empty query matches everything with score 0.
pub fn score(query: &str, candidate: &str) -> Option<i32> {
    let q: Vec<char> = query
        .chars()
        .filter(|c| !c.is_whitespace())
        .flat_map(char::to_lowercase)
        .collect();
    if q.is_empty() {
        return Some(0);
    }
    let cand: Vec<char> = candidate.chars().collect();
    let mut qi = 0;
    let mut total = 0;
    let mut last: Option<usize> = None;
    for (i, &c) in cand.iter().enumerate() {
        if qi == q.len() {
            break;
        }
        if !c.to_lowercase().eq(std::iter::once(q[qi])) {
            continue;
        }
        let word_start = i == 0 || !cand[i - 1].is_alphanumeric();
        total += 1;
        if word_start {
            total += 8;
        }
        match last {
            Some(l) if l + 1 == i => total += 5,
            Some(l) => total -= ((i - l - 1) as i32).min(5),
            None => total -= (i as i32).min(5),
        }
        last = Some(i);
        qi += 1;
    }
    (qi == q.len()).then_some(total)
}

/// Indices of `candidates` that match, best first; ties keep input order.
pub fn rank<S: AsRef<str>>(query: &str, candidates: &[S]) -> Vec<usize> {
    let mut scored: Vec<(i32, usize)> = candidates
        .iter()
        .enumerate()
        .filter_map(|(i, c)| score(query, c.as_ref()).map(|s| (s, i)))
        .collect();
    scored.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    scored.into_iter().map(|(_, i)| i).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subsequence_case_insensitive() {
        assert!(score("sav", "Save As…").is_some());
        assert!(score("SA", "save as").is_some());
        assert!(score("xyz", "Save").is_none());
        assert!(score("ss", "Save").is_none(), "each char used once");
        assert_eq!(score("", "anything"), Some(0));
        assert!(score("ää", "Älä tallenna").is_some(), "non-ASCII");
    }

    #[test]
    fn ranking_prefers_word_starts_and_runs() {
        let items = [
            "Reopen Closed Tab",
            "Close Tab",
            "Select All",
            "Delete Line",
        ];
        assert_eq!(rank("close", &items)[0], 1);
        assert_eq!(rank("ct", &items)[0], 1, "word starts: Close Tab");
        assert_eq!(rank("dl", &items), vec![3]);
        assert_eq!(rank("", &items), vec![0, 1, 2, 3]);
    }
}
