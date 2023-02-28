fn main(){
    
    let mut array = [4,2,6,11,9];

    for i in 0..array.len(){
        for j in i..array.len(){
            if array[i] > array[j]{
                let tmp = array[i];
                array[i] = array[j];
                array[j] = tmp;
            }
        }
    }

    println!("sorted: {:?}",array);
}
