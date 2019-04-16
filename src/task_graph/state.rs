#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TaskState {
    WaitingDependencies(usize), // TODO: Use it in the static scheduling algorithms too if it is useful
    Ready,
    Scheduled,  // Used by static sheduling algorithms only
    Processing, // Used by dynamic scheduling algorithms only
    Completed,  // Used by dynamic scheduling algorithms only
}
