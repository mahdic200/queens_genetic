pub mod chromosome;

use rand::Rng;
use std::rc::Rc;
use std::cell::RefCell;
use crate::genetic::chromosome::Chromosome;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Genetic {
    chromosome_length: usize,
    population_size: u32,
    per_mutation: f32,
    pub population: RefCell<Vec<Rc<Chromosome>>>,
    maxiter: u32,
}

impl Genetic {
    pub fn new(chromosome_length: usize, population_size: u32, per_mutation: f32, maxiter: u32) -> Genetic {
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

    fn init_population(&self) -> () {
        for _i in 1..=self.population_size {
            self.population.borrow_mut().push(Rc::clone(&self.random_chromosome()));
        }
    }

    pub fn random_chromosome(&self) -> Rc<Chromosome> {
        let mut rand_vec: Vec<usize> = vec![];
        let mut template: Vec<usize> = Vec::new();
        for i in 1..=self.chromosome_length {
            template.push(i as usize);
        }
        while template.len() > 0 {
            let remainder = template.len();
            let rn: usize = rand::thread_rng().gen_range(0..=30) % (remainder);
            rand_vec.push(template.remove(rn));
        }

        let chromosome = Chromosome::new(rand_vec);

        Rc::new(chromosome)
    }

    fn fitness_summation(&self) -> f32 {
        let mut sum: f32 = 0.0;
        for chromosome in &*self.population.borrow() {
            sum += &chromosome.fitness();
        }
        for chromosome in &*self.population.borrow() {
            chromosome.fitness_ratio(&sum);
        }
        sum
    }

    fn init_probability_range(&self) -> () {

        let mut temp: f32 = 0.0;
        for chromo in &*self.population.borrow() {
            let temp_ = temp + chromo.fitness_ratio(&1.0);
            chromo.set_probability_range([temp, temp_]);
            temp = temp_;
        }
    }

    pub fn parent_selection(&self) -> Vec<Rc<Chromosome>> {
        self.fitness_summation();

        self.init_probability_range();

        let mut new_parents: Vec<Rc<Chromosome>> = vec![];
        while new_parents.len() < self.population_size as usize {
            for chromo in &*self.population.borrow() {
                let rand_no = rand::thread_rng().gen_range(0.0..1.0);
                if chromo.is_chosen(rand_no) {
                    new_parents.push(Rc::clone(chromo));
                    break;
                }
            }
        }
        
        new_parents
    }
    // make this function private
    pub fn crossover(&self, parent_1: Rc<Chromosome>, parent_2: Rc<Chromosome>) -> (Rc<Vec<usize>>, Rc<Vec<usize>>) {
        let parent_1 = &parent_1.gens;
        let parent_2 = &parent_2.gens;

        let mut ind_1: usize = (rand::thread_rng().gen_range(0..100) % self.chromosome_length) as usize;
        let mut ind_2: usize = (rand::thread_rng().gen_range(0..100) % (self.chromosome_length - ind_1)) + ind_1 as usize;

        if ind_2 < ind_1 {let t = ind_2;ind_2 = ind_1;ind_1 = t;}

        let mut new_child_1: Vec<usize> = Vec::new();
        let mut new_child_2: Vec<usize> = Vec::new();
        for _i in 0..self.chromosome_length {
            new_child_1.push(0);
            new_child_2.push(0);
        }

        for i in ind_1..=ind_2 {
            new_child_1[i] = parent_1[i];
            new_child_2[i] = parent_2[i];
        }

        let mut poi_1: usize = (&ind_2 + 1) % self.chromosome_length;
        let mut poi_2: usize = (&ind_2 + 1) % self.chromosome_length;

        while new_child_1.contains(&0) {
            if !new_child_1.contains(&parent_2[poi_2]) {
                new_child_1[poi_1] = parent_2[poi_2];
                poi_1 = (poi_1 + 1) % self.chromosome_length;
            }
            poi_2 = (poi_2 + 1) % self.chromosome_length;
        }

        poi_1 = (&ind_1 + 1) % self.chromosome_length;
        poi_2 = (&ind_2 + 1) % self.chromosome_length;

        while new_child_2.contains(&0) {
            if !new_child_2.contains(&parent_1[poi_2]) {
                new_child_2[poi_1] = parent_1[poi_2];
                poi_1 = (poi_1 + 1) % self.chromosome_length;
            }
            poi_2 = (poi_2 + 1) % self.chromosome_length;
        }

        (Rc::new(new_child_1), Rc::new(new_child_2))
    }

    pub fn recombination(&self) {
        // self.crossover();
    }

}
