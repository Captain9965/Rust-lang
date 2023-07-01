fn main(){
    let mut v : Vec<u8> = vec![1, 2, 3];

    println!("{:?}", v);


    // to continue using v and avoid moving it into the for loop iteration methods, use a reference instead to borrow it later
    // this happens because the for loop takes ownership of the vector v
    for i in &v{
        println!("{i}");
    }

    // accessing elements:

    println!("The size of the vector indicating the number of elements that the vector can hold is -> {}", v.len());
    for i in 0..v.len(){
        // use get instead to return None if the value does not exist: 

        // println!("{}", v[i])
        println!("{:?}", v.get(i))

        /*
            the value returned here is some(num)??
            The value needs to be unwrapped 
        
         */
    }

    // Adding variables to a vector in Rust:

    v.push(20);
    println!("{:?}", v);

    // removing a value: specifies the index to remove
    v.remove(3);
    println!("{:?}", v);


    // initializing using new::

    println!("------------------ Using dynamic memory: --------------------");
    let mut new_vector : Vec<i32> = Vec::new();
    println!("{:?}", new_vector);

    // Add items:
    new_vector.push(20);
    println!("{:?}", new_vector);



}