use std::{sync::{Arc, Mutex}, cell::RefCell};

use super::{Notifier, proc_config::ProcessConfig};

pub struct ParExecutor<Out: Send + 'static> {
    res: RefCell<Option<Out>>,
    notify: Option<Notifier>,
}

impl<Out: Send + 'static> ParExecutor<Out> {
    /// Identical to default()
    pub fn new() -> Self {
        Self::default()
    }

    /// Create an Executor with a pre-installed notifyer
    pub fn with_notifyer(notifyer: impl FnMut() + Send + 'static) -> Self {
        Self {
            notify: Some(Box::new(notifyer)),
            ..Default::default()
        }
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
        // Run proc
        let out = (proc)(input);
        // Run notifier
        if keep_notifier {
            if let Some(not) = &mut self.notify {
                (not)();
            }
        } else if let Some(not) = new_notifier {
            (not)();
        }
        // Set res
        self.res = RefCell::new(Some(out));
    }

    /// Returns some if the processing task is done, returns none otherwise
    pub fn try_get_data(&self) -> Option<Out> {
        self.res.borrow_mut().take()
    }

    /// Blocks until the current processing task is done
    /// Returns None if no task is being processed
    pub fn get_data(&self) -> Option<Out> {
        self.res.borrow_mut().take()
    }

    /// Will ignore any results from processes started before this call
    pub fn clear_tasks(&self) {
        
    }
}
impl<Out: Send + 'static> Default for ParExecutor<Out> {
    /// Instantiates a ParExecutor that is ready to handle tasks
    fn default() -> Self {
        Self {
            res: RefCell::new(None),
            notify: None,    
        }
    }
}