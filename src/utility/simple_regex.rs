use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum RegexToken {
    Char(char),
    RepeatOp(char),
    Or(char),
    Bracket(char),
    Hyphen,
}

#[derive(Debug)]
pub struct SimpleRegex {
    trans: Vec<HashMap<char, usize>>,
    start: HashSet<usize>,
    accept: HashSet<usize>,
}

impl SimpleRegex {
    pub fn new(regex: &str) -> SimpleRegex {
        let tokens = Self::parse_into_token(regex);
        let (trans, start, accept) = Self::construct_automaton(&tokens);
        Self {
            trans,
            start,
            accept,
        }
    }

    fn parse_into_token(regex: &str) -> Vec<RegexToken> {
        let mut res = vec![];
        let s = regex.chars().collect::<Vec<char>>();

        {
            let mut i = 0;
            while i < s.len() {
                match s[i] {
                    '-' => res.push(RegexToken::Hyphen),
                    '[' | ']' => res.push(RegexToken::Bracket(s[i])),
                    '(' | ')' => (),
                    '?' | '*' | '+' => res.push(RegexToken::RepeatOp(s[i])),
                    '|' => res.push(RegexToken::Or(s[i])),
                    '\\' => {
                        i += 1;
                        res.push(RegexToken::Char(s[i]))
                    }
                    _ => res.push(RegexToken::Char(s[i])),
                }
                i += 1;
            }
        }

        res
    }

    fn construct_automaton(
        tokens: &[RegexToken],
    ) -> (Vec<HashMap<char, usize>>, HashSet<usize>, HashSet<usize>) {
        let mut trans = vec![HashMap::new()];
        let mut start = HashSet::new();
        let mut accept = HashSet::new();
        {
            let mut state = 0;
            let mut l = 0;
            start.insert(0);
            while l < tokens.len() {
                match tokens[l] {
                    RegexToken::Or('|') => {
                        accept.insert(state);
                        let new_state = trans.len();
                        start.insert(new_state);
                        state = new_state;
                        l += 1;
                    }
                    RegexToken::Bracket('[') => {
                        l += 1;
                        let mut r = l;
                        let mut chars = vec![];
                        while !matches!(tokens[r], RegexToken::Bracket(']')) {
                            if matches!(tokens[r + 1], RegexToken::Hyphen) {
                                let left = match tokens[r] {
                                    RegexToken::Char(c) => c,
                                    _ => unreachable!(),
                                };
                                let right = match tokens[r + 2] {
                                    RegexToken::Char(c) => c,
                                    _ => unreachable!(),
                                };
                                for c in left..=right {
                                    chars.push(c);
                                }
                                r += 3;
                            } else {
                                let c = match tokens[r] {
                                    RegexToken::Char(c) => c,
                                    _ => unreachable!(),
                                };
                                chars.push(c);
                                r += 1;
                            }
                        }
                        r += 1;
                        if r < tokens.len() && matches!(tokens[r], RegexToken::RepeatOp(_)) {
                            match tokens[r] {
                                RegexToken::RepeatOp('?') => {
                                    let next_state = trans.len();
                                    trans.push(HashMap::new());
                                    trans[state].insert('\0', next_state);
                                    chars.iter().for_each(|&c| {
                                        trans[state].insert(c, next_state);
                                    });
                                    state = next_state;
                                }
                                RegexToken::RepeatOp('*') => {
                                    chars.iter().for_each(|&c| {
                                        trans[state].insert(c, state);
                                    });
                                }
                                RegexToken::RepeatOp('+') => {
                                    let next_state = trans.len();
                                    trans.push(HashMap::new());
                                    chars.iter().for_each(|&c| {
                                        trans[state].insert(c, next_state);
                                        trans[next_state].insert(c, next_state);
                                    });
                                    state = next_state;
                                }
                                _ => unreachable!(),
                            }
                            r += 1;
                        } else {
                            let next_state = trans.len();
                            trans.push(HashMap::new());
                            chars.iter().for_each(|&c| {
                                trans[state].insert(c, next_state);
                            });
                            state = next_state;
                        }

                        l = r;
                    }
                    RegexToken::Char(c) => {
                        l += 1;
                        if l < tokens.len() && matches!(tokens[l], RegexToken::RepeatOp(_)) {
                            match tokens[l] {
                                RegexToken::RepeatOp('?') => {
                                    let next_state = trans.len();
                                    trans.push(HashMap::new());
                                    trans[state].insert('\0', next_state);
                                    trans[state].insert(c, next_state);
                                    state = next_state;
                                }
                                RegexToken::RepeatOp('*') => {
                                    trans[state].insert(c, state);
                                }
                                RegexToken::RepeatOp('+') => {
                                    let next_state = trans.len();
                                    trans.push(HashMap::new());
                                    trans[state].insert(c, next_state);
                                    trans[next_state].insert(c, next_state);
                                    state = next_state;
                                }
                                _ => unreachable!(),
                            }
                            l += 1;
                        } else {
                            let next_state = trans.len();
                            trans.push(HashMap::new());
                            trans[state].insert(c, next_state);
                            state = next_state;
                        }
                    }
                    _ => unreachable!(),
                }
            }
            accept.insert(state);
        }

        (trans, start, accept)
    }

    pub fn longest_match(&self, s: &[char], start: usize) -> usize {
        let mut dp = vec![false; self.trans.len()];
        self.start.iter().for_each(|&i| dp[i] = true);
        for j in 0..dp.len() {
            if dp[j] && self.trans[j].contains_key(&'\0') {
                dp[j + 1] = true;
            }
        }
        let mut last_match = start;
        for (i, &c) in s.iter().enumerate().skip(start) {
            let mut nxt = vec![false; self.trans.len()];
            for j in 0..dp.len() {
                if !dp[j] {
                    continue;
                }
                if self.trans[j].contains_key(&'\0') {
                    dp[j + 1] = true;
                }
                if let Some(&k) = self.trans[j].get(&c) {
                    nxt[k] = true;
                }
            }
            dp = nxt;

            if dp.iter().all(|&val| !val) {
                break;
            }
            for j in 0..dp.len() {
                if dp[j] && self.trans[j].contains_key(&'\0') {
                    dp[j + 1] = true;
                }
            }
            if dp
                .iter()
                .enumerate()
                .any(|(j, &val)| val && self.accept.contains(&j))
            {
                last_match = i + 1;
            }
        }

        last_match
    }
}
