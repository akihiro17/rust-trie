mod trie;
fn main() {
    println!("Hello, world!");

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

    assert_eq!(root.find("be"), 15);
    assert_eq!(root.find("bee"), 20);
    assert_eq!(root.find("beer"), 10);
    assert_eq!(root.find("best"), 35);
    assert_eq!(root.find("b"), 0);
    assert_eq!(root.find("bes"), 0);

    root.update_top();

    if let Some(top) = root.get_top("be") {
        for (r, t) in top {
            println!("{} {}", t, r.0);
        }
    }
    println!("---");
    if let Some(top) = root.get_top("b") {
        for (r, t) in top {
            println!("{} {}", t, r.0);
        }
    }
}
