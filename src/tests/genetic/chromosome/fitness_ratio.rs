use crate::genetic::chromosome::Chromosome;

#[test]
pub fn fitness_ratio() {
    let mut pop : Vec<Chromosome>= vec![
        Chromosome::new(vec![4, 8, 6, 1, 2, 3, 7, 5]),
        Chromosome::new(vec![4, 3, 1, 6, 5, 8, 7, 2]),
        Chromosome::new(vec![7, 8, 1, 4, 6, 3, 2, 5]),
        Chromosome::new(vec![7, 6, 4, 8, 2, 1, 3, 5]),
        Chromosome::new(vec![6, 8, 4, 1, 2, 5, 7, 3]),
        Chromosome::new(vec![7, 2, 8, 4, 1, 3, 6, 5]),
        Chromosome::new(vec![3, 6, 1, 2, 8, 5, 7, 4]),
        Chromosome::new(vec![7, 4, 5, 6, 1, 2, 3, 8]),
        Chromosome::new(vec![4, 5, 2, 8, 6, 1, 3, 7]),
        Chromosome::new(vec![1, 8, 3, 7, 4, 2, 5, 6]),
    ];
    // summation of all fitnesses 2.461544
    // it has 12 intersects
    // it has 0.083333336 fitness number
    assert_eq!(pop[9].fitness_ratio(&2.461544), 0.5 / 2.461544)
}