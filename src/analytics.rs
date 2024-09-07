use std::cell::{Cell, SyncUnsafeCell, UnsafeCell};
use std::pin::Pin;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::SystemTime;
use std::{mem, ptr};
use std::ops::{Deref, DerefMut};
use indicatif::{ProgressBar, ProgressStyle};

pub struct ReportHandler {
    start_time: SystemTime,
    progress_bar: ProgressBar,
    reporters: Vec<SyncUnsafeCell<Reporter>>
}

impl ReportHandler {
    pub fn new(desired_iterations: usize) -> Self {
        let bar = ProgressBar::new(desired_iterations as u64);
        bar.disable_steady_tick();
        // bar.abandon();
        bar.set_style(ProgressStyle::with_template(
            "[{elapsed}] {bar:40.green/blue} {pos}/{len} | remaining: (~{eta})"
        ).unwrap());

        ReportHandler {
            start_time: SystemTime::now(),
            progress_bar: bar,
            reporters: vec![]
        }
    }

    pub fn add_reporter(&mut self, desired_iterations: usize) -> Reporter {
        self.reporters.push(SyncUnsafeCell::new(Reporter::new(desired_iterations)));
        unsafe { self.reporters.get(0).unwrap().get().read() }
    }

    pub fn refresh(&mut self) {
        let reporters = self.reporters.iter().map(|x| unsafe { x.get().as_ref_unchecked() }).collect::<Vec<&Reporter>>();
        let new_position = reporters.iter().map(|x| { x.read_current_iterations() }).fold(0usize, |acc, x| {acc + x});
        let new_high_score = reporters.iter().map(|x| { x.read_high_score() }).fold(0usize, |acc, x| {usize::max(acc, x)});
        let new_wins = reporters.iter().map(|x| { x.read_wins() }).fold(0u8, |acc, x| {acc + x});

        // self.progress_bar.

        self.progress_bar.set_position(new_position as u64);
        self.progress_bar.tick();
    }
}

pub struct Reporter {
    desired_iterations: usize,
    current_iterations: ReadWhileWriting<usize>,
    high_score: ReadWhileWriting<usize>,
    wins: ReadWhileWriting<u8>
}

impl Reporter {
    pub fn new(desired_iterations: usize) -> Self {
        Reporter {
            desired_iterations,
            current_iterations: Default::default(),
            high_score: Default::default(),
            wins: Default::default(),
        }
    }

    pub fn read_current_iterations(&self) -> usize {
        self.current_iterations.read()
    }

    pub fn read_high_score(&self) -> usize {
        self.high_score.read()
    }

    pub fn read_wins(&self) -> u8 {
        self.wins.read()
    }

    pub fn write_current_iterations(&self) -> MutexGuard<'_, usize> {
        self.current_iterations.write()
    }

    pub fn write_high_score(&self) -> MutexGuard<'_, usize> {
        self.high_score.write()
    }

    pub fn write_wins(&self) -> MutexGuard<'_, u8> {
        self.wins.write()
    }
}

trait RWWSafe: Sized + Send + Unpin {}
impl <T: Sized + Send + Unpin> RWWSafe for T {}

// Sort of like how rust has smart-pointers, think of this as a dumb pointer!
// Never do this, never make this, never use this. It will never be worth the nanoseconds saved.
struct ReadWhileWriting<T>
where T: RWWSafe
{
    value: Pin<Box<SyncUnsafeCell<Mutex<T>>>>,
    reading_pointer: *const T
}

// This pointer is safe to send and sync ONLY because the user accepts that race conditions are an approved "feature" for reads, writes will still be safely guarded.
// The borrow checker still enforces all normal checks for writing, but reading is instead pointer-based against the pinned value.
unsafe impl<T: RWWSafe> Send for ReadWhileWriting<T> {}
unsafe impl<T: RWWSafe> Sync for ReadWhileWriting<T> {}

impl <T> ReadWhileWriting<T>
where T: RWWSafe
{
    fn new(value: T) -> Self {
        let mut selv = Self {
            value: Pin::new(Box::new(SyncUnsafeCell::new(Mutex::new(value)))),
            reading_pointer: ptr::null(),
        };
        selv.reading_pointer = ptr::addr_of!(*selv.value.get_mut().lock().unwrap().deref());
        selv
    }

    fn write(&self) -> MutexGuard<T> {
        unsafe { self.value.as_ref().get().as_mut_unchecked().lock().unwrap() }
    }

    fn read_ref(&self) -> &T {
        unsafe { self.reading_pointer.as_ref_unchecked() }
    }

    fn read(&self) -> T {
        unsafe { self.reading_pointer.read() }
    }
}

impl <T> Default for ReadWhileWriting<T>
where T: RWWSafe + Default
{
    fn default() -> Self {
        ReadWhileWriting::new(T::default())
    }
}

impl <T: RWWSafe> Deref for ReadWhileWriting<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.read_ref()
    }
}

// impl <T: RWWSafe> DerefMut for ReadWhileWriting<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut *self.write()
//     }
// }