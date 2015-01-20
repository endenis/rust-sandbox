// Based on gist https://gist.github.com/endenis/25b3b9e5cd3b3f002178
// Two implementations of insertion sort.

fn insertion_sort1(arr: &mut Vec<int>) -> &Vec<int> {
    for i in range(1, arr.len()) {
        let key = arr[i];
        let mut j = (i - 1) as int;
        let mut j_u = j as uint;
        while j >= 0 && arr[j_u] > key {
            arr[j_u + 1] = arr[j_u];
            j = j - 1;
            j_u = j as uint;
        }
        arr[j_u + 1] = key;
    }
    arr
}

fn insertion_sort2(arr: &mut Vec<int>) -> &Vec<int> {
    for i in range(1, arr.len()) {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            let buf = arr[j];
            arr[j] = arr[j - 1];
            arr[j - 1] = buf;
            j = j - 1;
        }
    }
    arr
}

fn main() {
    let mut vector1 = vec![5, 8, 6, 3, 7, 0, 4, 1];
    let mut vector2 = vector1.clone();
    println!("Original vector: {}", vector1);

    insertion_sort1(&mut vector1);
    println!("Insertion sort1: {}", vector1);

    insertion_sort2(&mut vector2);
    println!("Insertion sort2: {}", vector2);
}
