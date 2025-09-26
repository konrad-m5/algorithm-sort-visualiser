mod gui;

use gui::SortAlgorithm;


// Bubble sort
fn bubble_sort(arr: &mut Vec<i32>) -> &mut Vec<i32> {
    let mut swapped = true;

    while swapped {
        swapped = false;
        for i in 0..arr.len()-1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }
    }

    return arr;
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

