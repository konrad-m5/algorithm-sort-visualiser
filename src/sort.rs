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
      }// End inner if

    }// End of inner loop

    steps.push(SortingStep::SetSorted(n - i - 1));

    if !swapped {
      for k in 0..(n - i - 1) {
        steps.push(SortingStep::SetSorted(k));
      }
      break;
    }
  }

  steps.push(SortingStep::Finished);
  steps
}

// Quick sort
pub fn selection_sort(list: &mut Vec<i32>) -> Vec<SortingStep> {
  let mut steps = Vec::new();
  for i in 0..list.len() {
    
    let mut small = i;

    for j in (i + 1)..list.len() {
      steps.push(SortingStep::Compare(j, small));
      if list[j] < list[small] {
        small = j;
      }
    }
    if small != i {
      steps.push(SortingStep::Swap(i, small));
      list.swap(i, small);
    }

      steps.push(SortingStep::SetSorted(i));
  }

  steps.push(SortingStep::Finished);
  steps
}

// Insertion sort
pub fn insertion_sort(list: &mut Vec<i32>) -> Vec<SortingStep> {
  let mut steps = Vec::new();

  steps.push(SortingStep::SetSorted(0));

  for i in 1..list.len() {

    
    for j in (1..i + 1).rev() {
      steps.push(SortingStep::Compare(j - 1, j));
      if list[j - 1] <= list[j] { break; }

      list.swap(j - 1, j);
      steps.push(SortingStep::Swap(j - 1, j));
    }

    steps.push(SortingStep::SetSorted(i));
  }
  steps.push(SortingStep::Finished);
  steps
}
