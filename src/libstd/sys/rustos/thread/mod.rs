#[macro_use]
mod linked_list;
pub mod scheduler;
pub mod context;
pub mod stack;
mod fringe_wrapper;

pub use self::scheduler::Thread;

pub mod guard {
    pub unsafe fn current() -> usize { 0 }
    pub unsafe fn main() -> usize { 0 }
    pub unsafe fn init() {}
}