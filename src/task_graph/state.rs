#[derive(Clone, Copy)]
pub enum TaskState {
    Ready,
    Scheduled,
    Processing,
    Completed,
}
