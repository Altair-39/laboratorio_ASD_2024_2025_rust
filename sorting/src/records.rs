use csv::{ReaderBuilder, WriterBuilder};
use inquire::Select;
use std::error::Error;

use crate::mergesort::merge_sort;
use crate::quicksort::quick_sort;

#[derive(Debug, Clone)]
struct Record {
    name: String,
    value1: f64,
    value2: f64,
}

pub fn run_sorting_with_records(algorithm_choice: String) {
    // Prompt the user to choose a column to sort by
    let sort_column_choice =
        Select::new("Choose column to sort by", vec!["Name", "Value1", "Value2"])
            .prompt()
            .unwrap();

    // Read records from the CSV file
    let records = match read_csv("rsrc/records.csv") {
        Ok(records) => records,
        Err(e) => {
            println!("Failed to read CSV file: {}", e);
            return;
        }
    };

    // Sort the records based on the user's chosen column
    let sorted_records = match sort_column_choice {
        "Name" => sort_records_by_name(records, &algorithm_choice),
        "Value1" => sort_records_by_value1(records, &algorithm_choice),
        "Value2" => sort_records_by_value2(records, &algorithm_choice),
        _ => {
            println!("Invalid column selected.");
            return;
        }
    };

    if let Err(e) = write_csv("tmp/sorted_output.csv", sorted_records) {
        println!("Failed to write CSV file: {}", e);
    } else {
        println!("Sorted records have been written to 'sorted_output.csv'.");
    }
}

fn read_csv(file_path: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path)?;
    let mut records = Vec::new();

    for result in rdr.records() {
        let record = result?;

        let name = record[1].to_string();
        let value1 = match record[2].parse::<f64>() {
            Ok(v) => v,
            Err(_) => {
                continue;
            }
        };
        let value2 = match record[3].parse::<f64>() {
            Ok(v) => v,
            Err(_) => {
                continue;
            }
        };

        records.push(Record {
            name,
            value1,
            value2,
        });
    }
    Ok(records)
}

fn sort_records_by_name(mut records: Vec<Record>, algorithm_choice: &str) -> Vec<Record> {
    match algorithm_choice {
        "Merge Sort" => {
            merge_sort(&mut records, &|a: &Record, b: &Record| a.name.cmp(&b.name));
        }
        "Quick Sort" => {
            quick_sort(&mut records, &|a: &Record, b: &Record| a.name.cmp(&b.name));
        }
        _ => {
            println!("Invalid algorithm selected.");
            return records;
        }
    }
    records
}

fn sort_records_by_value1(mut records: Vec<Record>, algorithm_choice: &str) -> Vec<Record> {
    match algorithm_choice {
        "Merge Sort" => {
            merge_sort(&mut records, &|a: &Record, b: &Record| {
                a.value1.partial_cmp(&b.value1).unwrap()
            });
        }
        "Quick Sort" => {
            quick_sort(&mut records, &|a: &Record, b: &Record| {
                a.value1.partial_cmp(&b.value1).unwrap()
            });
        }
        _ => {
            println!("Invalid algorithm selected.");
            return records;
        }
    }
    records
}

fn sort_records_by_value2(mut records: Vec<Record>, algorithm_choice: &str) -> Vec<Record> {
    match algorithm_choice {
        "Merge Sort" => {
            merge_sort(&mut records, &|a: &Record, b: &Record| {
                a.value2.partial_cmp(&b.value2).unwrap()
            });
        }
        "Quick Sort" => {
            quick_sort(&mut records, &|a: &Record, b: &Record| {
                a.value2.partial_cmp(&b.value2).unwrap()
            });
        }
        _ => {
            println!("Invalid algorithm selected.");
            return records;
        }
    }
    records
}

fn write_csv(file_path: &str, records: Vec<Record>) -> Result<(), Box<dyn Error>> {
    let mut wtr = WriterBuilder::new()
        .has_headers(true)
        .from_path(file_path)?;

    wtr.write_record(["Name", "Value1", "Value2"])?;

    for record in records {
        wtr.write_record(&[
            record.name,
            record.value1.to_string(),
            record.value2.to_string(),
        ])?;
    }

    wtr.flush()?;
    Ok(())
}
