use anyhow::Context;
use chrono::NaiveDate;
use ics::{
    Alarm, Event, ICalendar,
    parameters::Related,
    properties::{DtStart, RRule, Summary, Trigger},
};
use std::{collections::HashMap, error::Error, io, str::FromStr};
use uuid::Uuid;

#[derive(Debug)]
struct Record {
    name: String,
    birthday: NaiveDate,
}

impl TryFrom<HashMap<String, String>> for Record {
    type Error = anyhow::Error;
    fn try_from(value: HashMap<String, String>) -> Result<Self, anyhow::Error> {
        let first_name = value
            .get("First Name")
            .map(String::to_owned)
            .unwrap_or_default();
        let last_name = value
            .get("Last Name")
            .map(String::to_owned)
            .unwrap_or_default();
        let birthday = value
            .get("Birthday")
            .map(|birthday| birthday.replace("--", "1900-"))
            .map(|birthday| NaiveDate::from_str(&birthday))
            .and_then(Result::ok)
            .context(format!(
                "No birthday found or impossible to parse {:?}",
                value
            ))?;

        Ok(Record {
            birthday,
            name: (first_name.to_owned() + " " + &last_name).trim().to_owned(),
        })
    }
}

fn read_birthdays() -> Result<Vec<Record>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let records = rdr
        .deserialize()
        .filter_map(|result| {
            let record: HashMap<String, String> = result.ok()?;
            let record: Result<Record, _> = record.try_into();
            record.ok()
        })
        .collect();

    Ok(records)
}

fn convert_to_ical(records: &[Record]) -> Result<(), std::io::Error> {
    let mut calendar = ICalendar::new("2.0", "ics-rs");

    records
        .iter()
        .map(|record| {
            let Record { birthday, name } = record;
            let birthday = birthday.format("%Y%m%d");
            let mut event = Event::new(Uuid::new_v4().to_string(), birthday.to_string());
            let start_date = DtStart::new(birthday.to_string());
            let summary = Summary::new(format!("Geburtstag {name}"));
            let rule = RRule::new("FREQ=YEARLY");

            let mut trigger = Trigger::new("PT8H");
            trigger.add(Related::End);
            let alarm = Alarm::audio(trigger);
            event.push(start_date);
            event.push(summary);
            event.push(rule);
            event.add_alarm(alarm);
            event
        })
        .for_each(|event| {
            calendar.add_event(event);
        });

    calendar.save_file("birthdays.ics")
}

fn main() -> Result<(), Box<dyn Error>> {
    let birthdays = read_birthdays()?;
    convert_to_ical(&birthdays)?;
    Ok(())
}
