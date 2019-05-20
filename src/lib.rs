pub mod emp;
pub mod time;
pub use emp::*;
pub use time::*;

pub fn schedule_employees(mut sched: Schedule, ros: Roster) -> Schedule {
    for (id, _empl) in ros.iter() {
        sched.assign_shift(id.clone(), 0, 9*60, 10*60);
    }

    sched
}
