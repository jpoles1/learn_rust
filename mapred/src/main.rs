//prepare for rust parallelism
extern crate rand;
extern crate num_cpus;
extern crate threadpool;
use std::thread;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use rand::Rng;
//implement map reduce in general form using threads...

//slave channels do parallel map (pmap) function
fn pmap(i:usize, map:fn() -> (f32, f32)) -> (f32, f32){
    println!("this is thread number {}", i);
    map()
}
//master reduces.
fn pred(res:(f32, f32)){
    println!("{:?}", res);
}
//run a map reduce task?
//here our map function will sum up a vector of values
//fn will return a tuple containing: (sum, length of vector)
fn main() {
  let ncpus = num_cpus::get();
  println!("This machine can run in parallel using {} cores/threads.", ncpus);
  let (tx, rx) = channel();
  let pool = ThreadPool::new(ncpus);
    for i in 0usize..ncpus {
        let txx = tx.clone();
       	pool.execute(move || {
            fn rngmap() -> (f32, f32){
                let mut rng = rand::thread_rng();
                let data_size = rng.gen_range(200, 2000);
                let data = (1..data_size).map(|_| {
                    rng.gen_range(0.0, 1.0f32)
                });
                let sum = data.fold(0.0, |agg:f32, x:f32| agg + x);
                (sum, data_size as f32)
                //println!("Sum: {}; Length: {}; Avg: {}", sum, data_size, sum/(data_size as f32));
            }
            txx.send(pmap(i, rngmap)).ok().expect("Unable to send message");
        });
        let res = rx.recv().unwrap();
        pred(res);
    }
    thread::sleep_ms(50);
}
