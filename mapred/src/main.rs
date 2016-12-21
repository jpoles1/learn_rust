//prepare for rust parallelism
extern crate num_cpus;
extern crate threadpool;
use std::thread;
use threadpool::ThreadPool;
//implement map reduce in general form using threads...

//slave channels do parallel map (pmap) function
fn pmap(map){

}
//master reduces.

//run a map reduce task?
//here our map function will sum up a vector of values
//fn will return a tuple containing: (sum, length of vector)
fn main() {
  let ncpus = num_cpus::get();
  println!("This machine can run in parallel using {} cores/threads.", ncpus);
  let pool = ThreadPool::new(ncpus);
    for i in 0..ncpus {
       	pool.execute(move || {
            println!("this is thread number {}", i)
       });
    }
    thread::sleep_ms(50);
}
