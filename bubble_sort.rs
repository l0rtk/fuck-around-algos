pub fn bubble_sort(array: &mut [i32]) {
    for i in 0..array.len() {
        for j in i..array.len() {
            if array[i] > array[j] {
                let _tmp = array[i];
                array[i] = array[j];
                array[j] = _tmp;
            }
        }
    }
}
