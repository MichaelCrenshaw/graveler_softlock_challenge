#![feature(sync_unsafe_cell)]
#![feature(ptr_as_ref_unchecked)]

use crate::naive::naive_main;

use std::time::{Duration, SystemTime};
use indicatif::*;
use crate::naive_optimized::naive_optimized_main;

mod naive;
mod analytics;
mod naive_optimized;

// Hyper-params (would be passed through args if I weren't lazy (also constants let the compiler do magic sometimes shh))
const ITERATION_COUNT: usize = 1_000_000_000;


// // If you're interested, a simple way to think of this is like a messaging service.
// // I allow any number of threads to return a message (in this case containing how many runs it has gone through), and increment the current iteration based on how many messages were received.
// #[cfg(debug_assertions)]
// static CHANNEL: (Sender<usize>, Receiver<usize>) = mpsc::channel();
// This type is guaranteed-aligned (as an atomic) AND is static, and therefore shouldn't (I hope) move around in memory. Thanks to that, we can get a slightly "safer" raw pointer later.
// static mut CURRENT_ITERATION: OnceLock<Arc<Mutex<AtomicUsize>>> = OnceLock::new();
// static mut CURRENT_ITERATION_MUTEX: Mutex<AtomicUsize> = Mutex::new(unsafe { CURRENT_ITERATION });

// This, my friends, is a raw pointer... don't use these.
// static ITERATION_POINTER: *const AtomicUsize = ptr::addr_of!(unsafe{ CURRENT_ITERATION });

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
