use std::sync::{Arc, Mutex, OnceLock};
use std::sync::atomic::AtomicUsize;
use fastrand::choice;
static mut CURRENT_ITERATION: OnceLock<Arc<Mutex<AtomicUsize>>> = OnceLock::new();

/// Runs in roughly six minutes (On my machine, while running other programs)
/// ```
/// Highest Ones Roll: 100
/// Number of Roll Sessions: 1000000000
/// Time elapsed: 355.7421444s
/// ```
pub fn naive_main() {
    unsafe { CURRENT_ITERATION.set(Arc::new(Mutex::new(AtomicUsize::new(0)))).expect("TODO: panic message"); }

    // Naive implementation, literal copy of your code.
    // Yeah, I'm talking to you Austin. Use random.randint(1, 4) next time I'm begging you.
    // Alright that's probably the end of my passive aggression, also it's purely a joke please don't stop coding; no one should be gate-keeping coding.
    // Gate-keeping coding is like gate-keeping writing, it's stupid and people should have gotten over it a long time ago.
    let items = vec![1usize, 2, 3, 4];
    let mut numbers = vec![0u64, 0, 0, 0];
    let mut rolls = 0usize;
    let mut max_ones = 0u64;

    // // Analytics information
    // let halt_threads = Arc::new(RwLock::new(false));

    // let mut progress_bar;
    // if cfg!(debug_assertions) {
    //     progress_bar = ProgressBar::new(ITERATION_COUNT as u64);
    //
    //     let ht = halt_threads.clone();
    //
    //     // Reporting "coroutine" (not a real one)
    //     let coro = move || {
    //         // This, my friends, is a raw pointer... don't use these. (I swear there's a "good" reason I'm using this instead of just passing clones of the progress bar around and incrementing)
    //         let iteration_pointer: *const usize = unsafe { ptr::addr_of!((*CURRENT_ITERATION.get().unwrap().lock().unwrap().get_mut())) };
    //         loop {
    //             if *ht.read().unwrap() {
    //                 progress_bar.finish_and_clear();
    //                 break;
    //             }
    //
    //             progress_bar.set_position(unsafe { *iteration_pointer.clone().as_ref().unwrap() as u64 });
    //         }
    //     };
    //
    //     thread::spawn(coro);
    // }

    while numbers[0] < 177 && rolls < 1_000_000_000 {
        numbers = vec![0, 0, 0, 0];
        for _ in 0..231 {
            let roll = *choice(&items).unwrap();
            numbers[roll - 1] = numbers[roll - 1] + 1;
        }
        rolls = rolls + 1;
        if numbers[0] > max_ones {
            max_ones = numbers[0];
        }
        // if cfg!(debug_assertions) {
        //     unsafe {
                // *CURRENT_ITERATION_MUTEX.get_mut().unwrap().get_mut() += 1;
                // *CURRENT_ITERATION.get().unwrap().get().unwrap().get() += 1;
                // *CURRENT_ITERATION.get().unwrap().lock().unwrap().get_mut() += 1;
            // }
        // }
    }

    // *halt_threads.write().unwrap() = true;

    println!("Highest Ones Roll: {max_ones}");
    println!("Number of Roll Sessions: {rolls}");
}