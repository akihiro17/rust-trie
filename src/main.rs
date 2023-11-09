use std::time::Instant;

use rand::Rng;
use random_word::Lang;

mod trie;
fn main() {
    let mut root = trie::Root::new_empty();

    let mut rng = rand::thread_rng();
    for _ in 0..10000000 {
        let word = random_word::gen(Lang::En);
        // println!("word: {}", word);
        root.add(word, rng.gen_range(0..10000));
    }
    let start = Instant::now();
    root.update_top();
    let end = start.elapsed();

    println!(
        "process time: {}.{:03}",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );

    if let Some(top_list) = root.get_top("twi") {
        println!("auto complete list for `twi`");
        for (r, t) in top_list {
            println!("{}: {}", t, r.0);
        }
    }
    if let Some(top_list) = root.get_top("bug") {
        println!("auto complete list for `bug`");
        for (r, t) in top_list {
            println!("{}: {}", t, r.0);
        }
    }
}
