/*fn bubble_sort(array: &mut [i32]) {
    let n = array.len();
    for i in 0..n {
        for j in 0..n-i-1 {
            if array[j] > array[j+1] {
                array.swap(j, j+1);
            }
        }
    }
}*/


fn bubble_sort<T: PartialOrd>(array: &mut [T]) {
    let n = array.len();

    for i in 0..n {
        for j in 0..n-i-1 {
            if array[j] > array[j+1] {
                array.swap(j, j+1);
            }
        }
    }
}

fn main() {
    //let mut array = vec![64, 34, 25, 12, 22, 11, 90];
    let mut array: [f64; 5] = [1.2, 3.4, 5.6, 7.8, 9.0];
    bubble_sort(&mut array);
    println!("Sorted array: {:?}", array);
}

