use rapid_phonetic_matcher::{Confidence, PhoneticMatcher};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // rpm <input> <target>
        3 => {
            let input = &args[1];
            let target = &args[2];
            let m = PhoneticMatcher::new();
            let score = m.calculate_similarity(input, target);
            let confidence = Confidence::from_score(score);
            println!("{:.3}  {:?}  {} vs {}", score, confidence, input, target);
        }
        // rpm <input> <target1> <target2> ...
        n if n >= 4 => {
            let input = &args[1];
            let candidates: Vec<&str> = args[2..].iter().map(|s| s.as_str()).collect();
            let m = PhoneticMatcher::new();
            let results = m.find_top_matches(input, &candidates, candidates.len());
            println!("input: {}\n", input);
            for (i, r) in results.iter().enumerate() {
                println!("  {}. {:.3}  {:?}  {}", i + 1, r.score, r.confidence, r.text);
            }
        }
        _ => {
            eprintln!("rpm - rapid phonetic matcher CLI\n");
            eprintln!("Usage:");
            eprintln!("  rpm <input> <target>                   Compare two strings");
            eprintln!("  rpm <input> <candidate1> <candidate2>  Rank candidates by similarity\n");
            eprintln!("Examples:");
            eprintln!("  rpm カルミ カルビ");
            eprintln!("  rpm IBM アイビーエム");
            eprintln!("  rpm かるみー カルビー カルピス カルディ");
            process::exit(1);
        }
    }
}