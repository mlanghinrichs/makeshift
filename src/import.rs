use csv;
// use super::emp::{ Employee, Roster };
use std::error::Error;
use std::fs;

pub fn get_roster() -> Result<(), Box<Error>> {
    let file = fs::File::open("../sched_lib/docs/roster.csv")?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        println!("{:#?}", record);
    }
    Ok(())
}

// fn hm_to_empl(hm: HashMap<String, String>) -> Employee {
    
// }