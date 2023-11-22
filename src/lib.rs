#[cfg(test)]
pub mod tests {
    fn fitness(queen_locations: Vec<usize>) -> usize {
        let mut fitness: usize = 0;
        for i in 0..queen_locations.len() - 1 {
            for j in &i + 1..queen_locations.len() {
                if &i.abs_diff(j) == &queen_locations[i].abs_diff(queen_locations[j]) {
                    fitness += 1;
                }
            }
        }

        fitness
    }
    

    #[test]
    fn fitness_calculation_on_best() {
        let locations = vec![4, 2, 7, 5, 1, 8, 6, 3];
        assert_eq!(fitness(locations), 0);
    }

    #[test]
    fn fitness_calculation_on_worse() {
        let locations = vec![4, 5, 6, 1, 2, 3, 8, 7];
        assert_eq!(fitness(locations), 10);
    }

    #[test]
    fn fitness_reverse_plus_epsilon() {
        let queen_locations: Vec<usize> = vec![7, 4, 2, 5, 8, 1, 3, 6];
        let mut fitness: f32 = fitness(queen_locations) as f32;
        fitness = 1.0 / (fitness + 0.00000001);
        assert_eq!(fitness, 100000000.0)
    }

}