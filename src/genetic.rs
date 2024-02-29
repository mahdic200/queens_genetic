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
    pub maxiter: u32,
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
    fn crossover(&self, parent_1: Rc<Chromosome>, parent_2: Rc<Chromosome>) -> (Rc<Chromosome>, Rc<Chromosome>) {
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

        let new_child_1 = Chromosome::new(new_child_1);
        let new_child_2 = Chromosome::new(new_child_2);

        (Rc::new(new_child_1), Rc::new(new_child_2))
    }

    fn recombination(&self, parents: &Vec<Rc<Chromosome>>) -> Vec<Rc<Chromosome>> {
        let mut offsprings: Vec<Rc<Chromosome>> = Vec::new();
        for i in (0..(parents.len() - 1)).step_by(2) {
            let (child1, child2) =
            self.crossover(
                Rc::clone(&parents[i]),
                Rc::clone(&parents[i + 1])
            );
            offsprings.push(Rc::clone(&child1));
            offsprings.push(Rc::clone(&child2));
        }
        
        let offsprings = offsprings;
        offsprings
    }

    fn swap_mutation(&self, chromosome: &Rc<Chromosome>) -> Rc<Chromosome> {
        let mut new_gens: Vec<usize> = chromosome.gens.clone();
        if rand::thread_rng().gen_range(0.0..1.0) <= self.per_mutation {
            let i = rand::thread_rng().gen_range(0..new_gens.len());
            let j = rand::thread_rng().gen_range(0..new_gens.len());
            let t = new_gens[i];
            new_gens[i] = new_gens[j];
            new_gens[j] = t;
        }

        let chromosome = Chromosome::new(new_gens);
        Rc::new(chromosome)
    }

    fn mutation(&self, offsprings: Vec<Rc<Chromosome>>) -> Vec<Rc<Chromosome>> {
        let mut offsprings = offsprings;
        for i in 0..offsprings.len() {
            offsprings[i] = self.swap_mutation(&offsprings[i]);
        }
        offsprings
    }

    pub fn maximum_fitness(&self, population: &Vec<Rc<Chromosome>>) -> (usize, f32) {
        let mut max_i = 0;
        let mut max_fit = population[0].fitness();
        for i in 0..population.len() {
            if population[i].fitness() > max_fit {
                max_fit = population[i].fitness();
                max_i = i;
            }
        }
        (max_i, max_fit)
    }

    pub fn start_loop(&self) -> (Rc<Chromosome>, Vec<f32>) {
        let mut best_fitnesses: Vec<f32> = Vec::new();
        let mut best: Rc<Chromosome> = Rc::new(Chromosome::new(vec![1, 2, 3, 4, 5, 6, 7, 8]));
        for i in 0..self.maxiter {
            let parents = self.parent_selection();
            let mut offsprings = self.recombination(&parents);
            offsprings = self.mutation(offsprings);
            self.population.replace(offsprings);
            let self_population = self.population.borrow_mut();
            let (best_index, best_fitness) = self.maximum_fitness(&self_population);
            best_fitnesses.push(best_fitness);
            if best.fitness() < self_population[best_index].fitness() {
                best = Rc::clone(&self_population[best_index]);
            }
            if i == self.maxiter - 1 {
                println!("last population : ");
                for chromo in &*self_population {
                    println!("g: {:?}, c: {:?}, f: {:?}", chromo.gens, chromo.intersects(), chromo.fitness());
                }
            }
        }
        (best, best_fitnesses)
    }
}
