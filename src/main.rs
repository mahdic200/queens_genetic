pub mod genetic;
use rand::Rng;
fn main() {
    use genetic::parent::*;
    let pop : Vec<Parent>= vec![
        Parent::new(vec![4, 8, 6, 1, 2, 3, 7, 5]),
        Parent::new(vec![4, 3, 1, 6, 5, 8, 7, 2]),
        Parent::new(vec![7, 8, 1, 4, 6, 3, 2, 5]),
        Parent::new(vec![7, 6, 4, 8, 2, 1, 3, 5]),
        Parent::new(vec![6, 8, 4, 1, 2, 5, 7, 3]),
        Parent::new(vec![7, 2, 8, 4, 1, 3, 6, 5]),
        Parent::new(vec![3, 6, 1, 2, 8, 5, 7, 4]),
        Parent::new(vec![7, 4, 5, 6, 1, 2, 3, 8]),
        Parent::new(vec![4, 5, 2, 8, 6, 1, 3, 7]),
        Parent::new(vec![1, 8, 3, 7, 4, 2, 5, 6]),
    ];

    let mut sum = 0.0;
    println!("{sum}");
    for iter in pop {
        sum += iter.fitness();
        iter.print();
    }
    println!("{sum}");
}

fn _rand_vec() -> Vec<u32> {
    let mut rand_vec: Vec<u32> = vec![];
    let mut template = vec![1, 2, 8, 4, 3, 5, 6, 7];
    while template.len() > 0 {
        let rn = rand::thread_rng().gen_range(0..template.len());
        rand_vec.push(template.remove(rn));
    }

    rand_vec
}