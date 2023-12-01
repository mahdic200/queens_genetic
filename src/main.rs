pub mod genetic;
pub mod plot;
use genetic::Genetic;

fn main() {

    let mut chr_len: String = String::new();
    println!("Enter queens number: ");
    std::io::stdin().read_line(&mut chr_len).unwrap();
    let chr_len = chr_len.trim().parse::<usize>().expect("invalid queens number");

    let mut pop_size: String = String::new();
    println!("Enter population size: ");
    std::io::stdin().read_line(&mut pop_size).unwrap();
    let pop_size = pop_size.trim().parse::<u32>().expect("invalid population size");

    let mut per_mut: String = String::new();
    println!("Enter permutation (between 0 and 1]): ");
    std::io::stdin().read_line(&mut per_mut).unwrap();
    let per_mut = per_mut.trim().parse::<f32>().expect("invalid queens number");

    let mut maxiter: String = String::new();
    println!("Enter maxiter (between 1 and 2000, suggested number: 100): ");
    std::io::stdin().read_line(&mut maxiter).unwrap();
    let maxiter = maxiter.trim().parse::<u32>().expect("invalid queens number");

    let genetic = Genetic::new(chr_len, pop_size, per_mut, maxiter);
    let (best, best_fitnesses) = genetic.start_loop();
    let mut points: Vec<(f32, f32)> = Vec::new();
    for i in 0..best_fitnesses.len() {
        points.push((i as f32, best_fitnesses[i] as f32));
    }
    println!("queens vector : {:?}", best.gens);
    println!("conflicts : {:?}", best.intersects());
    plot::draw(points, genetic.maxiter as f32).unwrap();
    println!("genetic_plot.png is your diagram. good bye !");
    let mut _final = String::new();
    std::io::stdin().read_line(&mut _final).unwrap();
}