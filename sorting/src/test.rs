use crate::compar::compare;
use crate::mergesort;
use crate::quicksort;

pub fn run_sorting_test_i32(algorithm_choice: &str, test_case_choice: &str) {
    match algorithm_choice {
        "Merge Sort" => {
            println!("You selected Merge Sort for i32");
            let data = get_test_case_data_i32(test_case_choice);
            let mut data_clone = data.clone();
            println!("Unsorted data: {:?}", data_clone);
            mergesort::merge_sort(&mut data_clone, &compare);
            println!("Sorted data: {:?}", data_clone);
        }
        "Quick Sort" => {
            println!("You selected Quick Sort for i32");
            let data = get_test_case_data_i32(test_case_choice);
            let mut data_clone = data.clone();
            println!("Unsorted data: {:?}", data_clone);
            quicksort::quick_sort(&mut data_clone, &compare);
            println!("Sorted data: {:?}", data_clone);
        }
        _ => {
            println!("Invalid algorithm selected.");
        }
    }
}

// Function to run the sorting test for f32
pub fn run_sorting_test_f32(algorithm_choice: &str, test_case_choice: &str) {
    match algorithm_choice {
        "Merge Sort" => {
            println!("You selected Merge Sort for f32");
            let data = get_test_case_data_f32(test_case_choice);
            let mut data_clone = data.clone();
            println!("Unsorted data: {:?}", data_clone);
            mergesort::merge_sort(&mut data_clone, &compare);
            println!("Sorted data: {:?}", data_clone);
        }
        "Quick Sort" => {
            println!("You selected Quick Sort for f32");
            let data = get_test_case_data_f32(test_case_choice);
            let mut data_clone = data.clone();
            println!("Unsorted data: {:?}", data_clone);
            quicksort::quick_sort(&mut data_clone, &compare);
            println!("Sorted data: {:?}", data_clone);
        }
        _ => {
            println!("Invalid algorithm selected.");
        }
    }
}

// Function to run the sorting test for String
pub fn run_sorting_test_string(algorithm_choice: &str, test_case_choice: &str) {
    match algorithm_choice {
        "Merge Sort" => {
            println!("You selected Merge Sort for String");
            let data = get_test_case_data_string(test_case_choice);
            let mut data_clone = data.clone();
            println!("Unsorted data: {:?}", data_clone);
            mergesort::merge_sort(&mut data_clone, &compare);
            println!("Sorted data: {:?}", data_clone);
        }
        "Quick Sort" => {
            println!("You selected Quick Sort for String");
            let data = get_test_case_data_string(test_case_choice);
            let mut data_clone = data.clone();
            println!("Unsorted data: {:?}", data_clone);
            quicksort::quick_sort(&mut data_clone, &compare);
            println!("Sorted data: {:?}", data_clone);
        }
        _ => {
            println!("Invalid algorithm selected.");
        }
    }
}

// Function to return test case data for i32
fn get_test_case_data_i32(test_case_choice: &str) -> Vec<i32> {
    match test_case_choice {
        "Test 1" => vec![3, 1, 4, 1, 5, 9],
        "Test 2" => vec![10, 20, 15, 5, 25],
        "Test 3" => vec![3, 1, 4, 1, 5, 9],
        _ => vec![],
    }
}

// Function to return test case data for f32
fn get_test_case_data_f32(test_case_choice: &str) -> Vec<f32> {
    match test_case_choice {
        "Test 1" => vec![3.1, 1.1, 4.4, 1.5, 5.9],
        "Test 2" => vec![10.2, 20.5, 15.7, 5.4, 25.8],
        "Test 3" => vec![3.2, 1.3, 4.6, 1.2, 5.1],
        _ => vec![],
    }
}

// Function to return test case data for String
fn get_test_case_data_string(test_case_choice: &str) -> Vec<String> {
    match test_case_choice {
        "Test 1" => vec![
            "apple".to_string(),
            "banana".to_string(),
            "cherry".to_string(),
        ],
        "Test 2" => vec!["dog".to_string(), "cat".to_string(), "elephant".to_string()],
        "Test 3" => vec![
            "orange".to_string(),
            "pear".to_string(),
            "grape".to_string(),
        ],
        _ => vec![],
    }
}
