use crate::records::run_sorting_with_records;
use crate::test::*;
use inquire::Select;

mod compar;
mod mergesort;
mod quicksort;
mod records;
mod test;

pub fn main() {
    let data_source_choice = Select::new("Choose data source", vec!["Test", "Records"])
        .prompt()
        .unwrap();

    let algorithm_choice =
        Select::new("Choose sorting algorithm", vec!["Merge Sort", "Quick Sort"])
            .prompt()
            .unwrap();

    match data_source_choice {
        "Test" => {
            let test_case_choice =
                Select::new("Choose test case", vec!["Test 1", "Test 2", "Test 3"])
                    .prompt()
                    .unwrap();

            let data_type_choice = Select::new("Choose data type", vec!["i32", "f32", "String"])
                .prompt()
                .unwrap();

            match data_type_choice {
                "i32" => run_sorting_test_i32(algorithm_choice, test_case_choice),
                "f32" => run_sorting_test_f32(algorithm_choice, test_case_choice),
                "String" => run_sorting_test_string(algorithm_choice, test_case_choice),
                _ => println!("Invalid data type selected."),
            }
        }
        "Records" => {
            let _ = run_sorting_with_records(algorithm_choice);
        }
        _ => println!("Invalid data source selected."),
    }
}
