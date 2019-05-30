use csv;
use super::emp::{ Employee, Roster };
use super::time::{ Day, Event, Time };
use std::error::Error;
use std::fs;

pub fn get_roster() -> Result<Roster, Box<Error>> {
    let file = fs::File::open("../sched_lib/docs/roster.csv")?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut ros = Roster::new();
    let mut headers = Vec::new();
    {
        for item in rdr.headers()? {
            headers.push(item.to_owned());
        }
    }
    for result in rdr.records() {
        let record = result?;
        if let Ok(empl) = build_empl(&record, &headers) {
            println!("{:#?}", empl);
            ros.add(empl);
        } else {
            println!("Error reading record: {:#?}", record);
        }
    }
    Ok(ros)
}

pub fn get_events() -> Result<Vec<Event>, Box<Error>> {
    let file = fs::File::open("../sched_lib/docs/events.csv")?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut out = Vec::new();
    for result in rdr.records() {
        let record = result?;
        if let Ok(event) = build_event(&record) {
            println!("{:#?}", event);
            out.push(event);
        } else {
            println!("Error reading record: {:#?}", record);
        }
    }
    Ok(out)
}

fn build_empl(sr: &csv::StringRecord, headers: &Vec<String>) -> Result<Employee, Box<Error>> {
    // id
    let mut empl = Employee::new(sr[0].to_owned());
    // cant_work_days
    for word in sr[1].split(", ") {
        if let Some(d) = Day::from_str(word) {
            empl.cant_work(d.to_index());
        }
    }
    // min_ and max_hours
    let min: usize = sr[2].parse()?;
    let max: usize = sr[3].parse()?;
    empl.change_hours(min, max)?;

    empl.add_role(&sr[4]);

    for i in 5..sr.len() {
        if &sr[i] != "" {
            empl.set_abil(&headers[i], sr[i].parse()?);
        }
    }

    Ok(empl)
}

fn build_event(sr: &csv::StringRecord) -> Result<Event, Box<Error>> {
    let name = sr[0].to_owned();
    let kind = sr[1].to_owned();
    let day = Day::from_str(&sr[2]).ok_or("bad day string")?;
    let start = Time::from_str(&sr[3]);
    let end = Time::from_str(&sr[4]);
    let num_emps: i32 = sr[7].parse()?;
    let mut req_emp_ids: Vec<String> = Vec::new();
    for empl in sr[8].split(", ") {
        req_emp_ids.push(empl.to_owned());
    }
    let out = Event { name, req_emp_ids, day, start, end, num_emps, kind };
    Ok(out)
}