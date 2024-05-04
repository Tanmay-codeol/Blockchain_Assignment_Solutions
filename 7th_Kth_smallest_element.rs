fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut i = low;
    for j in low..high {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, high);
    i
}

fn kth_smallest_helper(arr: &mut [i32], low: usize, high: usize, k: usize) -> i32 {
    if low == high {
        return arr[low];
    }

    let pivot_index = partition(arr, low, high);

    if pivot_index == k {
        arr[pivot_index]
    } else if pivot_index < k {
        kth_smallest_helper(arr, pivot_index + 1, high, k)
    } else {
        kth_smallest_helper(arr, low, pivot_index - 1, k)
    }
}

fn kth_smallest(arr: &mut [i32], k: usize) -> i32 {
    if k >= arr.len() {
        panic!("k should be less than the length of the array");
    }
    kth_smallest_helper(arr, 0, arr.len() - 1, k)
}

fn main() {
    let mut arr = [7, 10, 4, 3, 20, 15];
    let k = 3;
    let kth_smallest_element = kth_smallest(&mut arr, k - 1);
    println!("The {}th smallest element is: {}", k, kth_smallest_element);
}
