#![feature(async_closure)]
#![feature(sync_unsafe_cell)]
#![feature(ptr_as_ref_unchecked)]
extern crate core;

use std::error::Error;
use crate::naive::naive_main;

use std::time::{Duration, SystemTime};
use indicatif::*;
use crate::naive_optimized::naive_optimized_main;

mod naive;
mod analytics;
mod naive_optimized;

// Hyper-params (would be passed through args if I weren't lazy (also constants let the compiler do magic sometimes shh))
const ITERATION_COUNT: usize = 1_000_000_000;

// The performance is given in each file, as well as in comments below. But all numbers are based on my machine, with the following specs:
// Processor    :	AMD Ryzen 5 5600X 6-Core Processor, 3701 Mhz, 6 Core(s), 12 Logical Processor(s)
// Graphics Card:   Nvidia 3060 12gb VRAM // Only matters if I ever get around to the GPU-accelerated version I plan on making, so if this is here and I didn't make that then whoops, sorry.


fn main() {
    // Fun fact! Since pseudo-random number generation is deterministic--
    // and most pseudo-random number generators have an upper and lower bound to their seeds--
    // it may actually be provably impossible to win this fight with only the given Graveler.
    // For an example of something like this, take a look at why Minecraft cannot have end portal spawns with 12 eyes in unseeded runs. (I believe in all versions? But at least in recent versions)
    // Sadly, I do not know enough about the base code to know if this is the case; but I'd be fascinated to watch someone dive into the problem.

    // Startup
    // unsafe { CURRENT_ITERATION.set(Arc::new(Mutex::new(AtomicUsize::new(0)))).expect("TODO: panic message"); }


    // // This, my friends, is a raw pointer... don't use these.
    // let iteration_pointer: *const usize = unsafe { ptr::addr_of!((*CURRENT_ITERATION.get().unwrap().lock().unwrap().get_mut())) };

    // Some impl-agnostic metrics I want to keep track of
    let start_time: SystemTime = SystemTime::now();

    // naive_main();
    naive_optimized_main();

    println!("Time elapsed: {:?}", Duration::from(start_time.elapsed().unwrap()));
}
