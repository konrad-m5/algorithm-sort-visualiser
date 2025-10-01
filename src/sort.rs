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

// Selection sort
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

// Merge sort
pub fn merge_sort(list: &mut Vec<i32>) -> Vec<SortingStep> {
    let mut steps = Vec::new();
    let n = list.len();

    // Handle edge cases for empty or single-element lists
    if n <= 1 {
        if n == 1 {
            steps.push(SortingStep::SetSorted(0));
        }
        steps.push(SortingStep::Finished);
        return steps;
    }
    
    // Start the recursive merge sort process
    merge_sort_recursive_steps(list, 0, n, &mut steps);
    
    // Mark all elements as sorted at the end
    for i in 0..n {
        steps.push(SortingStep::SetSorted(i));
    }
    
    steps.push(SortingStep::Finished);
    steps
}

fn merge_sort_recursive_steps(
    list: &mut Vec<i32>,
    start: usize,
    end: usize,
    steps: &mut Vec<SortingStep>,
) {
    if end - start <= 1 {
        return;
    }
    
    let mid = start + (end - start) / 2;
    
    // Recursively sort left and right halves
    merge_sort_recursive_steps(list, start, mid, steps);
    merge_sort_recursive_steps(list, mid, end, steps);
    
    // Merge the two halves using a simple approach
    // This simulates the merge step
    let mut changed = true;
    while changed {
        changed = false;
        for i in start..end-1 {
            steps.push(SortingStep::Compare(i, i + 1));
            if list[i] > list[i + 1] {
                list.swap(i, i + 1);
                steps.push(SortingStep::Swap(i, i + 1));
                changed = true;
            }
        }
    }
}

// Add more sorting algorithms as needed

