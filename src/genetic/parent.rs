pub struct Parent {
    pub data: Vec<usize>,
    pub fitness: f32,
}

impl Parent {
    pub fn new(&self, data: Vec<usize>) -> Parent {
        Parent {
            data,
            fitness: self.fitness(&self.data)
        }
    }
    
    pub fn fitness(&self, queen_locations: &Vec<usize>) -> f32 {
        let mut fitness: f32 = 0.0;
        for i in 0..queen_locations.len() - 1 {
            for j in &i + 1..queen_locations.len() {
                if &i.abs_diff(j) == &queen_locations[i].abs_diff(queen_locations[j]) {
                    fitness += 1.0;
                }
            }
        }
        1.0 / (fitness + 0.00000001)
    }
}