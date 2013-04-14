extern mod core;
use core::rand::RngUtil;

/* Type definition for our genome (a vector of characters) */
type genome = ~[char];

fn main() {

  /* What string do we want to evolve into? Each character is a gene.
   * For now, this must be exactly 12 characters. */
  let ideal_genome: genome =str::to_chars("Hello world!");

  /* How often should mutations happen? (calculated per gene, not per genome)
   * 0 means never, 1 means always */
  let mutation_rate =0.05;

  let empty_genome = str::to_chars("            ");

  /* Keep track of how many generations it takes to evolve into ideal_genome */
  let mut gens: uint =0;

  /* Genomes for our mom, dad, and primordial ooze */
  let mut mom_genome: genome = empty_genome.clone();
  let mut dad_genome: genome = empty_genome.clone();
  let mut ooz_genome: genome;

  /* Fitness ratings ('ideal' fitness is 1.0) */
  let mut mom_fit =0.;
  let mut dad_fit =0.;
  let mut ooz_fit =0.;

  while ooz_fit !=1. {
    ooz_genome = breed(mom_genome, dad_genome);
    for ooz_genome.each_mut |gene| {
      if rand::random() %100 <=(mutation_rate *100. as uint) {
        *gene = mutate();
      }
    }
    ooz_fit = fitness(ooz_genome, ideal_genome);

    /* Print this genome */
    print(ooz_genome, ooz_fit);

    /* If our ooze has a high fitness, make it breed next generation */
    if ooz_fit >=mom_fit {
      mom_genome =ooz_genome.clone();
      mom_fit    =ooz_fit;
    } else if ooz_fit >=dad_fit {
      dad_genome =ooz_genome.clone();
      dad_fit    =ooz_fit;
    }
    gens +=1;
  }
  io::println(fmt!("Took %u generations", gens));
}

/* Returns a fitness for ooz_genome when compared to ideal_genome, from 0.0
 * (least fit) to 1.0 (most fit). */
fn fitness(ooz_genome: &[char], ideal_genome: &[char]) -> float {
  let     ideal_fit    =12.;
  let mut ooz_fit      =0.;
  for vec::each2(ooz_genome, ideal_genome) |ooz_gene, ideal_gene| {
    if ooz_gene ==ideal_gene {
      ooz_fit +=1.;
    }
  }
  ooz_fit /ideal_fit
}

/* Generates a random gene (char). */
fn mutate() -> char {
  rand::Rng().gen_char_from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz `1234567890!@#$%^&*()_+")
}

/* Returns a genome comprised of half of genome_a and half of genome_b. */
fn breed(genome_a: &[char], genome_b: &[char]) -> genome {
  let mut i =0;
  let mut child_genome =str::to_chars("            ");
  while i <12 {
    if i <12 /2 {
      child_genome[i] =genome_a[i];
    } else {
      child_genome[i] =genome_b[i];
    }
    i +=1;
  }
  child_genome
}

/* Prints an entire genome and its fitness level. */
fn print(g: &[char], fit: float) {
  for g.each |c| {
    io::print(fmt!("%c", *c));
  }
  io::println(fmt!(" (fitness: %f)", fit));
}
