pub mod chromosome;

use rand::Rng;
use std::rc::Rc;
use std::cell::RefCell;
use crate::genetic::chromosome::Chromosome;

#[allow(dead_code)]
pub struct Genetic {
    chromosome_length: u32,
    population_size: u32,
    per_mutation: f32,
    pub population: RefCell<Vec<Rc<Chromosome>>>,
    maxiter: u32,
}

impl Genetic {
    pub fn new(chromosome_length: u32, population_size: u32, per_mutation: f32, maxiter: u32) -> Genetic {
        if per_mutation > 1.0 {
            panic!("per_mutation argument could not be greater than 1 !");
        }
        let genetic = Genetic {
            chromosome_length,
            population_size,
            per_mutation,
            population: RefCell::new(vec![]),
            maxiter,
        };
        genetic.init_population();
        genetic
    }

    pub fn init_population(&self) -> () {
        for _i in 1..=self.population_size {
            self.population.borrow_mut().push(Rc::clone(&self.random_chromosome()));
        }
    }

    pub fn random_chromosome(&self) -> Rc<Chromosome> {
        let mut rand_vec: Vec<usize> = vec![];
        let mut template = vec![1, 2, 8, 4, 3, 5, 6, 7];
        while template.len() > 0 {
            let rn = rand::thread_rng().gen_range(0..template.len());
            rand_vec.push(template.remove(rn));
        }

        let chromosome = Chromosome::new(rand_vec);

        Rc::new(chromosome)
    }

}
