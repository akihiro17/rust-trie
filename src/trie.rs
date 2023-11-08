use std::{cmp::Reverse, collections::HashMap};

type Link = Box<Node>;
pub struct Node {
    pub key: char,
    next: HashMap<char, Link>,
    pub count: u64,
    pub top: Vec<(Reverse<u64>, String)>,
    pub value: Option<String>,
}
pub struct Root {
    root: HashMap<char, Link>,
}

impl Node {
    pub fn new(key: char) -> Link {
        Box::new(Node {
            key,
            next: HashMap::new(),
            count: 0,
            top: vec![(Reverse(0), "".to_string()); 0],
            value: None,
        })
    }

    pub fn update(&mut self) {
        if let Some(value) = &self.value {
            self.top.push((Reverse(self.count), value.clone()));
        }
        for (_, n) in self.next.iter_mut() {
            n.update();

            let mut t2 = n.top.clone();
            self.top.append(&mut t2);
            self.top.sort();
            self.top.resize(5, (Reverse(0), "".to_string()));
        }
    }
}

impl Root {
    pub fn new_empty() -> Root {
        Root {
            root: HashMap::new(),
        }
    }

    pub fn add(&mut self, word: &str) {
        let mut len = word.len();
        let value = word.to_string();

        let mut word = word.chars();
        if let Some(start) = word.next() {
            len -= 1;

            let mut n = self.root.entry(start).or_insert(Node::new(start));

            for (i, c) in word.enumerate() {
                let tmp = n.next.entry(c).or_insert(Node::new(c));
                n = tmp;
                if i == len - 1 {
                    n.count += 1;
                    n.value = Some(value.clone());
                }
            }
        }
    }

    pub fn find(&self, word: &str) -> u64 {
        let mut word = word.chars();
        if let Some(start) = word.next() {
            self.root.get(&start).map_or(0, |mut n| {
                for c in word {
                    match n.next.get(&c) {
                        Some(ref tmp) => n = tmp,
                        None => break,
                    }
                }
                n.count
            })
        } else {
            0
        }
    }

    pub fn get_top(&self, word: &str) -> Option<&Vec<(Reverse<u64>, String)>> {
        let mut word = word.chars();
        if let Some(start) = word.next() {
            self.root.get(&start).map_or(None, |mut n| {
                for c in word {
                    match n.next.get(&c) {
                        Some(ref tmp) => n = tmp,
                        None => break,
                    }
                }
                Some(&n.top)
            })
        } else {
            None
        }
    }

    pub fn update_top(&mut self) {
        for (_, node) in self.root.iter_mut() {
            node.update();
        }
    }
}

mod tests {
    use std::cmp::Reverse;

    use crate::trie;

    fn root() -> trie::Root {
        let mut root = trie::Root::new_empty();

        for _ in 0..15 {
            root.add("be");
        }
        for _ in 0..20 {
            root.add("bee");
        }
        for _ in 0..29 {
            root.add("bet");
        }
        for _ in 0..14 {
            root.add("buy");
        }
        for _ in 0..10 {
            root.add("beer");
        }
        for _ in 0..35 {
            root.add("best");
        }
        for _ in 0..11 {
            root.add("win");
        }

        root
    }

    #[test]
    fn test_find() {
        struct TestCase {
            str: String,
            expected: u64,
        }

        let root = root();

        let tests = [
            TestCase {
                str: "b".to_string(),
                expected: 0,
            },
            TestCase {
                str: "be".to_string(),
                expected: 15,
            },
            TestCase {
                str: "bee".to_string(),
                expected: 20,
            },
            TestCase {
                str: "beer".to_string(),
                expected: 10,
            },
            TestCase {
                str: "best".to_string(),
                expected: 35,
            },
        ];

        for t in tests {
            let v = root.find(&t.str);
            assert_eq!(
                t.expected, v,
                "[{}] expected: {}, got: {}",
                t.str, t.expected, v,
            );
        }
    }

    #[test]
    fn test_update_top() {
        struct TestCase {
            str: String,
            expected: Vec<(Reverse<u64>, String)>,
        }
        let tests = [
            TestCase {
                str: "be".to_string(),
                expected: vec![
                    (Reverse(35), "best".to_string()),
                    (Reverse(29), "bet".to_string()),
                    (Reverse(20), "bee".to_string()),
                    (Reverse(15), "be".to_string()),
                    (Reverse(10), "beer".to_string()),
                ],
            },
            TestCase {
                str: "b".to_string(),
                expected: vec![
                    (Reverse(35), "best".to_string()),
                    (Reverse(29), "bet".to_string()),
                    (Reverse(20), "bee".to_string()),
                    (Reverse(15), "be".to_string()),
                    (Reverse(14), "buy".to_string()),
                ],
            },
        ];

        let mut root = root();
        root.update_top();
        for t in tests {
            assert_eq!(Some(&t.expected), root.get_top(&t.str));
        }
    }
}
