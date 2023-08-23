fn main() {
    let mut a = [61, 8, 15, 22, 3, 73, 11, 84, 45];
    //let mut a = [85.6, 2.1, 3.5, 8.4, 66.4];
    // bubble_sort_i32(&mut a);
    bubble_sort_template(&mut a);
    for i in a.iter() {
        println!("{}", i);
    }  
}

// fn bubble_sort_i32(array: &mut [i32]) {
//     let size = array.len();
//     if size <= 1 {
//         return;
//     }

//     for i in 0..(size - 1) {
//         for j in 1..(size - i) {
//             if array[j - 1] > array[j] {
//                 array.swap(j - 1, j);
//             }
//         }
//     }
// }

fn bubble_sort_template<T: PartialOrd>(array: &mut [T]) {
    let size = array.len();
    if size <= 1 {
        return;
    }

    for i in 0..(size - 1) {
        for j in 1..(size - i) {
            if array[j - 1] > array[j] {
                array.swap(j - 1, j);
            }
        }
    }
}