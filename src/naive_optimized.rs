use std::cell::SyncUnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex, MutexGuard};
use fastrand;
use crate::analytics::{ReportHandler, Reporter};
use std::thread;
use num_cpus;
use crate::ITERATION_COUNT;

pub fn naive_optimized_main() {

    let mut report_handler = ReportHandler::new(ITERATION_COUNT);

    let logical_cores = num_cpus::get();
    let processors = logical_cores.checked_sub(2).unwrap_or(1usize);
    println!("cores: {logical_cores} \n procs: {processors}");


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
    let mut stop_reports = false;
    let reporting_handle = thread::spawn(move || {
        loop {
            report_handler.refresh();
        }
    });

    handles.into_iter().for_each(|x| { x.join().unwrap(); });
    stop_reports = true;
    reporting_handle.join().unwrap();
}

pub fn run_simulations(desired_iterations: usize, reporter: Reporter) {
    let mut iteration_guard: MutexGuard<usize> = reporter.write_current_iterations();
    let mut most_ones_guard: MutexGuard<usize> = reporter.write_high_score();
    let mut wins_guard: MutexGuard<u8> = reporter.write_wins();

    let iteration_reporter: &mut usize = iteration_guard.deref_mut();
    let most_ones_reporter: &mut usize = most_ones_guard.deref_mut();
    let wins_reporter: &mut u8 = wins_guard.deref_mut();

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
        if local_iteration % 1000 == 0 {
            *iteration_reporter = local_iteration;
            *most_ones_reporter = local_most_ones;
            *wins_reporter = local_wins;
        }
    }
}