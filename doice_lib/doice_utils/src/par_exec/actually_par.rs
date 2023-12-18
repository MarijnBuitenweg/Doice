use std::{
    cell::Cell,
    sync::{
        atomic::{AtomicU32, Ordering},
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
};

use super::{proc_config::ProcessConfig, Notifier};

/// Executes tasks yielding Out in parallel, and yields only the most recently started one
/// It adds approx 50us to the execution time of the given task, so it's best to use it for expensive computations
pub struct ParExecutor<Out: Send + 'static> {
    /// Receiver for valid results
    fin_rx: Receiver<Out>,
    /// Transmitter for valid results
    fin_tx: Sender<Out>,
    /// Logic that is to be executed when a valid result is generated
    notify: Option<Arc<Mutex<Notifier>>>,
    /// Sequence number of the next expected result
    seq: Arc<AtomicU32>,
    /// Is the Executor processing something?
    /// This is present to make sure the blocking getter does not try to wait for nothing (which may cause deadlocks)
    busy: Cell<bool>,
}

impl<Out: Send + 'static> ParExecutor<Out> {
    /// Identical to default()
    pub fn new() -> Self {
        Self::default()
    }

    /// Create an Executor with a pre-installed notifyer
    pub fn with_notifyer(notifyer: impl FnMut() + Send + 'static) -> Self {
        Self {
            notify: Some(Arc::new(Mutex::new(Box::new(notifyer)))),
            ..Default::default()
        }
    }

    /// Helper to increment the seq
    fn inc_seq(&self) {
        self.seq.fetch_add(1, Ordering::SeqCst);
    }

    /// Processes the given input using its implementation of Into<Out>
    pub fn process_into<In: Send + 'static + Into<Out>>(
        &mut self,
        input: In,
    ) -> ProcessConfig<In, fn(In) -> Out, Out> {
        self.process_with(input, In::into)
    }

    /// Processes the given input into Out by applying the given function/closure
    pub fn process_with<In, Exec>(&mut self, input: In, proc: Exec) -> ProcessConfig<In, Exec, Out>
    where
        In: Send + 'static,
        Exec: FnOnce(In) -> Out + Send + 'static,
    {
        ProcessConfig::new(input, proc, self)
    }

    pub(crate) fn process_helper<In, Exec>(
        &mut self,
        new_notifier: &mut Option<Notifier>,
        keep_notifier: bool,
        input: In,
        proc: Exec,
    ) where
        In: Send + 'static,
        Exec: FnOnce(In) -> Out + Send + 'static,
    {
        // Set the notifyer properly
        if let Some(not) = new_notifier.take() {
            if let Some(old_not) = &mut self.notify {
                *old_not.lock().unwrap() = not;
            } else {
                self.notify = Some(Arc::new(Mutex::new(not)));
            }
        } else if !keep_notifier {
            self.notify = None;
        }

        // Set self to busy
        self.busy.set(true);
        // Update the seq, and make a copy for the processing thread
        self.inc_seq();
        let task_seq = self.seq.load(Ordering::SeqCst);
        let current_seq = self.seq.clone();
        // Clone the sender, so the processing thread can send its results to the receive/filter thread
        let sender = self.fin_tx.clone();
        // Make a copy of the notifyer
        let notify_handle = self.notify.clone();
        rayon::spawn(move || {
            // Do the work
            let data = (proc)(input);

            if current_seq.load(Ordering::SeqCst) == task_seq {
                if let Some(notify) = notify_handle {
                    (notify.lock().unwrap())();
                }
                let _ = sender.send(data);
            }
        });
    }

    /// Returns some if the processing task is done, returns none otherwise
    pub fn try_get_data(&self) -> Option<Out> {
        if let Ok(val) = self.fin_rx.try_recv() {
            self.busy.set(false);
            Some(val)
        } else {
            None
        }
    }

    /// Blocks until the current processing task is done
    /// Returns None if no task is being processed
    pub fn get_data(&self) -> Option<Out> {
        if self.busy.get() {
            self.busy.set(false);
            Some(self.fin_rx.recv().unwrap())
        } else {
            None
        }
    }

    /// Will ignore any results from processes started before this call
    pub fn clear_tasks(&self) {
        self.inc_seq();
    }
}

impl<Out: Send + 'static> Default for ParExecutor<Out> {
    /// Instantiates a ParExecutor that is ready to handle tasks
    fn default() -> Self {
        // Initialize the channel for final results
        let (fin_tx, fin_rx) = channel();

        Self {
            fin_rx,
            fin_tx,
            notify: None,
            seq: Arc::new(AtomicU32::default()),
            busy: Cell::new(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    #[test]
    fn startup_time_test() {
        let ts = Instant::now();
        let mut exec = ParExecutor::<f64>::with_notifyer(|| println!("Notify!"));
        println!("Setup took {:#?}", ts.elapsed());
        let ts = Instant::now();
        exec.process_into(1.32f32);
        println!("Processing took {:#?}", ts.elapsed());
        println!("Converting to f64: {:#?}", exec.get_data());
    }

    extern crate test;

    // #[bench]
    // fn startup_bench(b: &mut test::Bencher) {
    //     b.iter(|| {
    //         test::black_box(ParExecutor::<f64>::new());
    //     });
    // }

    // #[bench]
    // fn proc_bench(b: &mut test::Bencher) {
    //     let mut exec = ParExecutor::<f64>::new();
    //     b.iter(|| {
    //         for i in 0..1000 {
    //             exec.process_into(i as f32);
    //             test::black_box(
    //                 exec.get_data()
    //                     .expect("no process in flight after process was started"),
    //             );
    //         }
    //     });
    // }
}
