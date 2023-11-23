#[derive(Debug)]
pub struct Chromosome {
    pub gens: Vec<usize>,
    fitness: Option<f32>,
    fitness_ratio: Option<f32>,
}

impl Chromosome {
    pub fn new(gens: Vec<usize>) -> Chromosome {
        let new_instance = Chromosome {
            gens,
            fitness: None,
            fitness_ratio: None,
        };
        new_instance.fitness();
        new_instance
    }

    pub fn intersects(&self) -> f32 {
        let mut fitness: f32 = 0.0;
        for i in 0..self.gens.len() - 1 {
            for j in &i + 1..self.gens.len() {
                if &i.abs_diff(j) == &self.gens[i].abs_diff(self.gens[j]) {
                    fitness += 1.0;
                }
            }
        }
        fitness
    }

    pub fn fitness(&self) -> f32 {
        if let Some(fitness) = self.fitness {
            fitness
        }
        else {
            1.0 / (self.intersects() + self.epsilon())
        }
    }

    pub fn fitness_ratio(&mut self, summation: &f32) -> f32 {
        if let Some(fitness_ratio) = self.fitness_ratio {
            fitness_ratio
        }
        else {
            self.fitness_ratio = Some(&self.fitness() / summation);
            &self.fitness() / summation
        }
    }

    pub fn epsilon(&self) -> &f32 {
        &0.00000001
    }

    pub fn print(&self) {
        #[derive(Debug)]
        #[allow(unused)]
        struct Print<T, U> {
            gens: T,
            intersects: U,
            fitness: U,
        }
        let print = Print {
            gens: &self.gens,
            intersects: &self.intersects(),
            fitness: &self.fitness(),
        };

        println!("{:?}", print);
    }
}

