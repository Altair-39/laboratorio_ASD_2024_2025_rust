use inquire::Select;
use memmap2::Mmap;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::time::Instant;

use crate::mergesort::merge_sort;
use crate::quicksort::quick_sort;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Record {
    name: String,
    value1: i64,
    value2: f64,
    line_range: (usize, usize),
}

pub fn run_sorting_with_records(algorithm_choice: &str) -> Result<(), Box<dyn Error>> {
    let sort_column_choice = Select::new(
        "Choose a column to sort by",
        vec!["Name", "Value1", "Value2"],
    )
    .prompt()?;

    let start_total = Instant::now();

    let (mmap, mut records) = {
        let start_reading = Instant::now();
        let file = File::open("rsrc/records.csv")?;
        let mmap = unsafe { Mmap::map(&file)? };
        let records = parse_csv_mmap(&mmap)?;
        println!("Reading and parsing time: {:.4?}", start_reading.elapsed());
        (mmap, records)
    };

    let sort_start = Instant::now();
    match sort_column_choice {
        "Name" => sort_records(&mut records, algorithm_choice, |a, b| a.name.cmp(&b.name)),
        "Value1" => sort_records(&mut records, algorithm_choice, |a, b| {
            a.value1.partial_cmp(&b.value1).unwrap_or(Ordering::Equal)
        }),
        "Value2" => sort_records(&mut records, algorithm_choice, |a, b| {
            a.value2.partial_cmp(&b.value2).unwrap_or(Ordering::Equal)
        }),
        _ => eprintln!("Invalid column selected."),
    }
    println!("Sorting time: {:.4?}", sort_start.elapsed());

    let write_start = Instant::now();
    write_sorted_csv("tmp/sorted_output.csv", &records, &mmap)?;
    println!("Writing time: {:.4?}", write_start.elapsed());

    println!("Total execution time: {:.4?}", start_total.elapsed());
    Ok(())
}

fn parse_csv_mmap(mmap: &Mmap) -> Result<Vec<Record>, Box<dyn Error>> {
    let bytes = mmap.as_ref();
    let num_chunks = rayon::current_num_threads() * 4;
    let chunk_size = bytes.len() / num_chunks;

    let chunk_boundaries: Vec<usize> = (0..num_chunks)
        .map(|i| {
            let pos = i * chunk_size;
            if pos == 0 {
                0
            } else {
                bytes[pos..]
                    .iter()
                    .position(|&b| b == b'\n')
                    .map(|p| pos + p + 1)
                    .unwrap_or(bytes.len())
            }
        })
        .collect();

    let records: Vec<Record> = chunk_boundaries
        .par_windows(2)
        .flat_map(|window| {
            let chunk_start = window[0];
            let chunk = &bytes[window[0]..window[1]];
            let mut records = Vec::new();
            let mut pos = 0;

            while pos < chunk.len() {
                let line_start = pos;
                while pos < chunk.len() && chunk[pos] != b'\n' {
                    pos += 1;
                }

                if pos > line_start {
                    let absolute_start = chunk_start + line_start;
                    let absolute_end = chunk_start + pos - 1;

                    let line = &chunk[line_start..pos];
                    let mut fields = line.split(|&b| b == b',');

                    fields.next();

                    if let (Some(name), Some(value1_bytes), Some(value2_bytes)) =
                        (fields.next(), fields.next(), fields.next())
                    {
                        let name = String::from_utf8_lossy(name).into_owned();
                        let value1 = parse_i64(value1_bytes).unwrap_or(0);
                        let value2 = parse_f64(value2_bytes).unwrap_or(f64::NAN);

                        records.push(Record {
                            name,
                            value1,
                            value2,
                            line_range: (absolute_start, absolute_end),
                        });
                    }
                }

                pos += 1;
            }

            records
        })
        .collect();

    Ok(records)
}
fn parse_i64(bytes: &[u8]) -> Option<i64> {
    std::str::from_utf8(bytes).ok()?.parse().ok()
}
fn parse_f64(bytes: &[u8]) -> Option<f64> {
    fast_float::parse(bytes).ok()
}

fn sort_records<F>(records: &mut [Record], algorithm: &str, cmp: F)
where
    F: Fn(&Record, &Record) -> Ordering,
{
    match algorithm {
        "Merge Sort" => merge_sort(records, &cmp),
        "Quick Sort" => quick_sort(records, &cmp),
        _ => eprintln!("Invalid algorithm selected."),
    }
}

fn write_sorted_csv(
    output_path: &str,
    records: &[Record],
    mmap: &Mmap,
) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(output_path)?;
    let mut writer = std::io::BufWriter::with_capacity(1024 * 1024 * 32, file);

    let chunks: Vec<_> = records
        .par_chunks(1000)
        .map(|chunk| {
            let mut buffer = Vec::with_capacity(1024 * 128);
            for record in chunk {
                buffer.extend_from_slice(&mmap[record.line_range.0..=record.line_range.1]);
                if !buffer.ends_with(b"\n") {
                    buffer.push(b'\n');
                }
            }
            buffer
        })
        .collect();

    for chunk in chunks {
        writer.write_all(&chunk)?;
    }

    Ok(())
}
