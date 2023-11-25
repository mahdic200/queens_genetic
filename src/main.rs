pub mod genetic;
use std::rc::Rc;

fn main() {
    use genetic::Genetic;

    let genetic = Genetic::new(8, 2, 0.1, 10);
    let pop = &*genetic.population.borrow();
    let _c = genetic.crossover(Rc::clone(&pop[0]), Rc::clone(&pop[1]));
}