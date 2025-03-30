use inquire::Select;
use memmap2::Mmap;
use rayon::prelude::*;
use smallstr::SmallString;
use std::cmp::Ordering;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::time::Instant;

use crate::compar::{compare_f64_quicksort, compare_str_lex};
use crate::mergesort::merge_sort;
use crate::quicksort::quick_sort;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

type SmallStr = SmallString<[u8; 32]>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Record {
    name: SmallStr,
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
        let records = parse_csv(&mmap)?;
        println!("Reading and parsing time: {:.4?}", start_reading.elapsed());
        (mmap, records)
    };

    let sort_start = Instant::now();
    match sort_column_choice {
        "Name" => sort_records(&mut records, algorithm_choice, |a, b| {
            compare_str_lex(&a.name, &b.name)
        }),
        "Value1" => sort_records(&mut records, algorithm_choice, |a, b| {
            a.value1.partial_cmp(&b.value1).unwrap_or(Ordering::Equal)
        }),
        "Value2" => sort_records(&mut records, algorithm_choice, |a, b| {
            compare_f64_quicksort(a.value2, b.value2)
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

#[cfg(target_arch = "x86_64")]
unsafe fn find_commas_simd(bytes: &[u8]) -> (Option<usize>, Option<usize>, Option<usize>) {
    let mut comma1 = None;
    let mut comma2 = None;
    let mut comma3 = None;
    let comma = _mm_set1_epi8(b',' as i8);
    let mut i = 0;

    while i + 16 <= bytes.len() {
        let chunk = _mm_loadu_si128(bytes.as_ptr().add(i) as *const __m128i);
        let eq = _mm_cmpeq_epi8(chunk, comma);
        let mask = _mm_movemask_epi8(eq) as u32;

        if mask != 0 {
            for j in 0..16 {
                if mask & (1 << j) != 0 {
                    let pos = i + j;
                    if comma1.is_none() {
                        comma1 = Some(pos);
                    } else if comma2.is_none() {
                        comma2 = Some(pos);
                    } else if comma3.is_none() {
                        comma3 = Some(pos);
                        return (comma1, comma2, comma3);
                    }
                }
            }
        }
        i += 16;
    }

    for (j, &b) in bytes[i..].iter().enumerate() {
        if b == b',' {
            let pos = i + j;
            if comma1.is_none() {
                comma1 = Some(pos);
            } else if comma2.is_none() {
                comma2 = Some(pos);
            } else if comma3.is_none() {
                comma3 = Some(pos);
                break;
            }
        }
    }

    (comma1, comma2, comma3)
}

fn parse_csv(mmap: &Mmap) -> Result<Vec<Record>, Box<dyn Error>> {
    let bytes = mmap.as_ref();
    let num_chunks = rayon::current_num_threads();
    let chunk_size = (bytes.len() + num_chunks - 1) / num_chunks;

    let chunk_boundaries: Vec<usize> = (0..=num_chunks)
        .map(|i| {
            let pos = i * chunk_size;
            if pos == 0 || pos >= bytes.len() {
                pos.min(bytes.len())
            } else {
                let mut p = pos;
                while p < bytes.len() && bytes[p] != b'\n' {
                    p += 1;
                }
                if p < bytes.len() {
                    p + 1
                } else {
                    p
                }
            }
        })
        .collect();

    let records: Vec<Record> = chunk_boundaries
        .par_windows(2)
        .flat_map(|window| {
            let chunk_start = window[0];
            let chunk_end = window[1];
            let mut records = Vec::with_capacity(1024);
            let mut pos = chunk_start;

            while pos < chunk_end {
                let line_start = pos;
                while pos < chunk_end && bytes[pos] != b'\n' {
                    pos += 1;
                }

                if pos > line_start {
                    let line_end = pos;
                    pos += 1;

                    let (comma1, comma2, comma3) =
                        unsafe { find_commas_simd(&bytes[line_start..line_end]) };

                    if let (Some(c1), Some(c2), Some(c3)) = (comma1, comma2, comma3) {
                        let name_bytes = &bytes[line_start + c1 + 1..line_start + c2];
                        let name = match std::str::from_utf8(name_bytes) {
                            Ok(s) => SmallStr::from(s),
                            Err(_) => SmallStr::from(String::from_utf8_lossy(name_bytes).as_ref()),
                        };

                        let value1 = unsafe {
                            std::str::from_utf8_unchecked(
                                &bytes[line_start + c2 + 1..line_start + c3],
                            )
                            .parse()
                            .unwrap_or(0)
                        };

                        let value2 = unsafe {
                            std::str::from_utf8_unchecked(&bytes[line_start + c3 + 1..line_end])
                                .parse()
                                .unwrap_or(f64::NAN)
                        };

                        records.push(Record {
                            name,
                            value1,
                            value2,
                            line_range: (line_start, line_end),
                        });
                    }
                }
            }

            records
        })
        .collect();

    Ok(records)
}

fn sort_records<F>(records: &mut [Record], algorithm: &str, cmp: F)
where
    F: Fn(&Record, &Record) -> Ordering + Sync,
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
        .create(true)
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
