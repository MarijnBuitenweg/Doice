use super::{ParExecutor, Notifier};

/// Only exists briefly while configuring a new processing task for a ParExecutor
pub struct ProcessConfig<'a, In, Exec, Out>
where
    In: Send + 'static,
    Out: Send + 'static,
    Exec: FnOnce(In) -> Out + Send + 'static,
{
    new_notifyer: Option<Notifier>,
    keep_notifier: bool,
    input: Option<In>,
    task: Option<Exec>,
    executor: &'a mut ParExecutor<Out>,
}

impl<'a, In, Exec, Out> ProcessConfig<'a, In, Exec, Out>
where
    In: Send + 'static,
    Out: Send + 'static,
    Exec: FnOnce(In) -> Out + Send + 'static,
{
    pub fn new(input: In, task: Exec, executor: &'a mut ParExecutor<Out>) -> Self {
        ProcessConfig {
            new_notifyer: None,
            keep_notifier: false,
            input: Some(input),
            task: Some(task),
            executor,
        }
    }

    pub fn keep_notifier(mut self) -> Self {
        self.keep_notifier = true;
        self
    }

    pub fn with_notifyer(mut self, notifyer: Notifier) -> Self {
        self.new_notifyer = Some(notifyer);
        self
    }
}

impl<'a, In, Exec, Out> Drop for ProcessConfig<'a, In, Exec, Out>
where
    In: Send + 'static,
    Out: Send + 'static,
    Exec: FnOnce(In) -> Out + Send + 'static,
{
    /// Makes sure the notifyer is removed if necessary
    fn drop(&mut self) {
        self.executor.process_helper(
            &mut self.new_notifyer,
            self.keep_notifier,
            self.input.take().unwrap(),
            self.task.take().unwrap(),
        );
    }
}