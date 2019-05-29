use csv;
use super::emp::{ Employee, Roster };
use super::time::Day;
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