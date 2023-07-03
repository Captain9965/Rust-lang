fn main(){
    let numbers = vec![2, 3, 4];
    // we have to call collect()
    let new : Vec<_>= numbers.iter().map(|i| i + 1).collect();

    println!("{:?}", new);
}