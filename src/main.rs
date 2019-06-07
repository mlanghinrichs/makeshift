use makeshift::{
    self, import,
    time::{Day, Schedule},
};

fn main() {
    let ros = import::get_roster().unwrap();
    let mut sched = get_schedule();
    let evs = import::get_events().unwrap();
    sched.events.extend(evs);
    sched.assign_required_shifts(&ros);
    println!("{}", ros);
    println!("{}", sched);
    sched.expand_shifts("Matt".to_string());
}

/// Return the full week's schedule for Labyrinth.
fn get_schedule() -> Schedule {
    let mut sched = Schedule::new();
    sched.set_hours(Day::Saturday, 9, 21);
    sched.set_hours(Day::Sunday, 10, 18);
    sched.set_hours(Day::Tuesday, 10, 22);
    sched.set_hours(Day::Wednesday, 10, 21);
    sched.set_hours(Day::Thursday, 10, 22);
    sched.set_hours(Day::Friday, 10, 22);

    sched
}
