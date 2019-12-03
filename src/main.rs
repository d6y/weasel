use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

const TARGET: &str = "METHINKSITISAWEASEL";
const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn random_word<R: Rng + ?Sized>(rng: &mut R) -> String {
    (0..TARGET.len())
        .map(|_| {
            let i = rng.gen_range(0, ALPHABET.len());
            ALPHABET[i] as char
        })
        .collect()
}

fn count_correct(word: &str) -> usize {
    word.chars()
        .zip(TARGET.chars())
        .filter(|(a, b)| a == b)
        .count()
}

fn main() {
    assert!(TARGET.len() == 19);
    assert!(ALPHABET.len() == 26);

    let mut rng: StdRng = SeedableRng::seed_from_u64(1);

    let sample_size = 1_000_000;

    let at_least_one_correct = (0..sample_size)
        .map(|_| random_word(&mut rng))
        .map(|w| count_correct(&w))
        .filter(|&c| c > 0)
        .count();

    let fraction = at_least_one_correct as f64 / sample_size as f64;
    println!("{:1.4}", fraction);
}
