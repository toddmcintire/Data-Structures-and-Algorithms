fn main() {
    let my_arr = [61, 82, 67, 4, 98, 20, 37, 85];

    println!("{:?}", insertion_sort(my_arr));
}

fn insertion_sort(mut a: [i32; 8]) -> [i32;8]{
    let n = a.len();
    let mut i = 1;

    while i < n {
        let current = a[i];
        let mut j = i as i32 -1;

        while j >= 0 && a[j as usize] > current {
            a[(j + 1) as usize] = a[j as usize];
            j = j -1;
        }
        a[(j + 1) as usize] = current;

        i = i + 1;
    }

    a
}