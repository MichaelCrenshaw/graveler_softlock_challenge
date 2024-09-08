use std::pin::Pin;
use std::sync::Arc;
use std::time::SystemTime;
use std::ptr;
use std::ops::{Deref, DerefMut};
use indicatif::{ProgressBar, ProgressStyle};

pub struct ReportHandler {
    start_time: SystemTime,
    progress_bar: ProgressBar,
    reporters: Vec<Pin<Arc<Reporter>>>
}

impl ReportHandler {
    pub fn new(desired_iterations: usize, planned_cycles: usize) -> Self {
        let bar = ProgressBar::new(desired_iterations as u64);
        bar.set_style(ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.green/blue} {pos}/{len} | remaining: (~{eta})\n {msg}"
        ).unwrap());

        ReportHandler {
            start_time: SystemTime::now(),
            progress_bar: bar,
            reporters: vec![]
        }
    }

    pub fn add_reporter(&mut self, desired_iterations: usize) -> Pin<Arc<Reporter>> {
        self.reporters.push(Pin::new(Arc::new(Reporter::new(desired_iterations))));
        let len = self.reporters.len();
        unsafe { self.reporters.get_unchecked(len - 1).clone() }
    }

    pub fn refresh(&mut self) {
        let new_position = self.reporters.iter().map(|x| { x.read_current_iterations() }).fold(0usize, |acc, x| {acc + x});
        let new_high_score = self.reporters.iter().map(|x| { x.read_high_score() }).fold(0usize, |acc, x| { usize::max(acc, x) });
        let new_wins = self.reporters.iter().map(|x| { x.read_wins() }).fold(0u8, |acc, x| {acc + x});

        self.progress_bar.set_message(format!("Current high score: {new_high_score} | Current wins: {new_wins}"));
        self.progress_bar.set_position(new_position as u64);
    }

    pub fn close(&mut self) {
        self.progress_bar.finish()
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

    pub fn write_current_iterations(&self) -> &mut usize {
        self.current_iterations.write()
    }

    pub fn write_high_score(&self) -> &mut usize {
        self.high_score.write()
    }

    pub fn write_wins(&self) -> &mut u8 {
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
    value: Pin<Box<T>>,
    pointer: Option<*mut T>
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
            value: Pin::new(Box::new(value)),
            pointer: None,
        };
        selv.pointer = Option::Some(ptr::addr_of_mut!(*selv.value));
        selv
    }

    fn write(&self) -> &mut T {
        // Yeah, completely screw safety now lol
        unsafe { self.pointer.unwrap().as_mut_unchecked() }
    }

    fn read_ref(&self) -> &T {
        unsafe { self.pointer.unwrap().as_ref_unchecked() }
    }

    fn read(&self) -> T {
        unsafe { self.pointer.unwrap().read() }
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

impl <T: RWWSafe> DerefMut for ReadWhileWriting<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.write()
    }
}