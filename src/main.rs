#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

//use std::time::{SystemTime, UNIX_EPOCH};
use std::time::Instant;

mod lambolt;
mod parser;
mod runtime;
mod text;

use runtime as rt;

fn main() {
  //let mut worker = rt::new_worker();
  //worker.size = 1;
  //worker.node[0] = rt::Cal(0, 0, 0);
  //let start = Instant::now();
  //rt::normal(&mut worker, 0);
  //let total = (start.elapsed().as_millis() as f64) / 1000.0;
  //println!("* rwts: {} ({:.2}m rwt/s)", worker.cost, (worker.cost as f64) / total / 1000000.0);
  //println!("* time: {:?}", total);

  println!(
    "{}",
    text::text_to_utf8(&text::highlight(
      3,
      7,
      &text::utf8_to_text(
        "oi tudo bem? como vai você hoje?\neu pessoalmente estou ok.\nespero que vc tbm"
      )
    ))
  );
  println!(":pp");
}