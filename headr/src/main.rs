use rand::seq::SliceRandom; // 0.7.2

fn main() {

    let mut vs = vec![0, 1, 2, 3, 4];
    vs.shuffle(&mut rand::thread_rng());
    println!("{:?}", vs);

   if let Err(e) = headr::get_args().and_then(headr::run) {
     eprintln!("{}", e);
     std::process::exit(1);
   }

}
