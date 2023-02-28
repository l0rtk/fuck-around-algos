fn main(){
    
    let mut array = [4,2,6,10,11,9,0];
    println!("original: {:?}",array);
    let mut swaps = 0;

    for i in 0..array.len(){
        for j in i..array.len(){
            if array[i] > array[j]{
                let tmp = array[i];
                array[i] = array[j];
                array[j] = tmp;
                swaps += 1;
            }
        }
    }

    println!("sorted: {:?}",array);
    println!("swaps: {}",swaps);
}
