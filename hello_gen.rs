extern mod core;
use core::rand::RngUtil;

fn main() {

  /* What string do we want to evolve into? Each character is a gene.
   * For now, this must be exactly 12 characters. */
  let ideal_genome =str::to_chars("Hello world!");

  /* How often should mutations happen? (calculated per gene, not per genome)
  /* 0 means never, 1 means always */
  let mutation_rate =0.05;

  /* Keep track of how many generations it takes to obtain 'ideal'! */
  let mut gens: uint = 0;

  /* Genomes for our mom, dad, and primordial ooze */
  let mut mom_genome: ~[char] =str::to_chars("            ");
  let mut dad_genome: ~[char] =str::to_chars("            ");
  let mut ooz_genome: ~[char];

  /* Fitness ratings ('ideal' fitness is 1.0) */
  let mut mom_fit =0.;
  let mut dad_fit =0.;
  let mut ooz_fit =0.;

  while ooz_fit != 1. {
    ooz_genome = breed(mom_genome, dad_genome);
    for ooz_genome.eachi |k, v| {
      if rand::random() %100 <=(mutation_rate *100. as uint) {
        ooz_genome[k] = mutate();
      }
    }
    ooz_fit = fitness(ooz_genome, ideal_genome);

    /* Print this genome */
    for ooz_genome.each |c| {
      io::print(fmt!("%c", *c));
    }
    io::println(fmt!(" (fitness: %f)", ooz_fit));

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

fn fitness(ooz_genome: &[char], ideal_genome: &[char]) -> float {
  let     ideal_fit    =12.;
  let mut ooz_fit      =0.;
  for ooz_genome.eachi |k, v| {
    if ideal_genome[k] ==*v {
      ooz_fit +=1.;
    }
  }
  ooz_fit /ideal_fit
}

fn mutate() -> char {
  rand::Rng().gen_char_from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz `1234567890!@#$%^&*()_+")
}

fn breed(genome_a: &[char], genome_b: &[char]) -> ~[char] {
  let mut i =0;
  let mut child_genome: ~[char] =str::to_chars("            ");
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
