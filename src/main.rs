use rand::seq::IndexedRandom;
use combinatorial::factorial;
use rational::Rational;

fn nck(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    }
    factorial(n) / (factorial(k) * factorial(n - k))
}

const BOOKS : [u8;10] = [1,2,3,4,5,6,7,8,9,10];
const TOTAL_TRIALS: usize = 1000000;

fn main() {
    let mut counter = [0;6];
    
    for _ in 0..TOTAL_TRIALS {
        let mut rng: rand::prelude::ThreadRng = rand::rng();
        let books1 = BOOKS.choose_multiple(&mut rng, 5).into_iter().collect::<Vec<&u8>>();
        let books2 = BOOKS.choose_multiple(&mut rng, 5).into_iter().collect::<Vec<&u8>>();
        counter[
            books1
                .iter()
                .filter(|&x| books2.contains(&x))
                .count()
        ] += 1;
    }
    println!("Counter: {:?}", counter);
    println!("Expected: {:?}", (0..=5)
                                    .map(|x| 
                                        (TOTAL_TRIALS as f64) * (nck(5, x) * nck(5, 5 - x)) as f64 / nck(10, 5) as f64
                                    )
                                    .collect::<Vec<f64>>()
    );
    println!("Probabilities: {:?}", (0..=5)
                                    .map(|x| 
                                        format!("{}",Rational::new((nck(5, x) * nck(5, 5 - x)) as u64, nck(10, 5) as u64))
                                    )
                                    .collect::<Vec<String>>()
    );
}
