use crate::gui::SortAlgorithm;
use crate::gui::SortingStep;


// Bubble sort
pub fn bubble_sort(list: &mut Vec<i32>) -> Vec<SortingStep> {

  let n = list.len();
  let mut steps = Vec::new();

  for i in 0..n {
    let mut swapped = false;
    for j in 0..(n - i - 1) {
      steps.push(SortingStep::Compare(j, (j + 1)));
      if list[j] > list[j + 1] {
        list.swap(j, j + 1);
        steps.push(SortingStep::Swap(j, (j + 1)));
        swapped = true;
      }
    }
    if !swapped {
      break;
    }
  }

  steps.push(SortingStep::Finished);
  steps
}

// Quick sort
fn selection_sort(list: &mut [i64]) {
  for i in 0..list.len() {

    let mut small = i;
    for j in (i + 1)..list.len() {

      if list[j] < list[small] {
        small = j;

      }
    }

    list.swap(small, i);
  }
}

// Insertion sort
fn insertion_sort<T: Ord>(list: &mut [T]) {
  for i in 1..list.len() {
    for j in (1..i + 1).rev() {
      if list[j - 1] <= list[j] { break; }
      list.swap(j - 1, j)
    }
  }
}

