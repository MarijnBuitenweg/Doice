pub struct SerialExecutor<Out: Send + 'static> {
    result: Option<Out>,
}