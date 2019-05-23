pub mod emp;
pub mod time;

pub fn assign_required_shifts(ros: &emp::Roster, mut sched: time::Schedule) -> time::Schedule {
    let mut assignments: Vec<(String, time::Event)> = Vec::new();
    for event in sched.get_events() {
        if event.has_reqs() {
            for emp_id in event.req_ids() {
                assignments.push((emp_id.clone(), event.clone()));
            }
        }
    }
    for (id, ev) in assignments {
        sched.assign_event(id, ev);
    }
    sched
}