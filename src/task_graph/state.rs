#[derive(Clone, Copy)]
pub enum TaskState {
    WaitingDependencies,
    Ready,
    Scheduled,
    Processing,
    Completed,
}
