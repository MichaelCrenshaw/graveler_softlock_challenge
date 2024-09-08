use std::pin::Pin;
use std::sync::Arc;
use fastrand;
use std::sync::mpsc::channel;
use crate::analytics::{ReportHandler, Reporter};
use std::thread;
use std::time::Duration;
use num_cpus;
use crate::ITERATION_COUNT;


/// Runs in roughly one minute (On my cpu, while running other programs)
/// ```
/// Highest Ones Roll: 99
/// Number of Roll Sessions: 1000000000
/// Time elapsed: 58.6822327s
/// ```
pub fn naive_optimized_main() {
    let logical_cores = num_cpus::get();
    let processors = logical_cores.checked_sub(1).unwrap_or(1usize);

    let mut report_handler = ReportHandler::new(ITERATION_COUNT, processors);

    // split the jobs
    let cycles_per_core = ITERATION_COUNT / logical_cores;
    let correction = ITERATION_COUNT - (cycles_per_core * processors);

    let mut handles = vec![];

    for num in 0..processors {
        let cycles = if num == 0 {correction + cycles_per_core} else {cycles_per_core};
        let reporter = report_handler.add_reporter(cycles);

        handles.push(thread::spawn(move || {
            run_simulations(cycles, reporter)
        }));
    }

    // Reporting thread
    let (sender, receiver) = channel();
    let reporting_handle = thread::spawn(move || {
        loop {
            // See if there is a new message, otherwise loop
            if let Ok(_) = receiver.recv_timeout(Duration::from_millis(100)) {
                report_handler.close();
                break
            }

            report_handler.refresh();
        }
    });

    handles.into_iter().for_each(|x| { x.join().unwrap(); });
    sender.send(()).unwrap();
    reporting_handle.join().unwrap();
}

pub fn run_simulations(desired_iterations: usize, reporter: Pin<Arc<Reporter>>) {
    let iteration_reporter: &mut usize = reporter.write_current_iterations();
    let most_ones_reporter: &mut usize = reporter.write_high_score();
    let wins_reporter: &mut u8 = reporter.write_wins();

    let mut local_iteration = 0usize;
    let mut local_most_ones = 0usize;
    let mut local_wins = 0u8;

    // It's literally faster to not check if we've won yet... so we don't. Instead, we just report how many wins happened (which will always be 0, obviously)
    while local_iteration < desired_iterations {
        let mut current_ones = 0usize;
        for _ in 0..231 {
            // Fun fact, any optimizer worth its assembler can rationalize this into branchless code
            if fastrand::u8(1..=4) == 1 { current_ones += 1}
        }
        if current_ones == 231 {
            local_wins += 1
        }
        if current_ones > local_most_ones {
            local_most_ones = current_ones;
        }
        local_iteration += 1;

        // Hoping for some excellent loop unraveling here
        if local_iteration % 100_000 == 0 {
            *iteration_reporter = local_iteration;
            *most_ones_reporter = local_most_ones;
            *wins_reporter = local_wins;
        }
    }

    *iteration_reporter = local_iteration;
    *most_ones_reporter = local_most_ones;
    *wins_reporter = local_wins;
}