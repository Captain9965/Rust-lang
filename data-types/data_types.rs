fn main(){

    /* integer type:  */
    let a : i8 = 100;
    println!("The integer is {a}");


    /* unsigned int type:  */

    let t : u16 = 64000;
    println!("The 16 bit integer is {t}");

    /* float we have f32 and f64 which is more precise:  */

    let f : f64 = 56.0;
    println!("The float number is {f}");

    let mut m : bool = false;
    println!("The boolean value is {m}");

    /* character:  */

    let mut c : char = 'j';
    println!("The characeter is {c}");

    /* type referencing...one does not have to specify the data type:  */

    let mut x = "Name";
    // x = 3; // does not work

    println!("The infered variable is {x}");

    /* arrays without data type: */

    let arr = [4, 5, 7, 9, 10];
    let item = arr[0];

    println!("First item is -> {item}");

    /* loop over the array */
    for i in arr{
        println!("{i}");
    }

    /* initilizing an array with data type:  */
    
    let arr2: [i32; 5] = [5, 6, 34, 56, 89];
    println!("{:?}", arr2);

    /* array with default values with a repeat expression:  */

    let arr3: [i32; 5] = [4;5];
    println!("{:?}", arr3);

    /* or:  */

    let arr4 = [20; 10];
    println!("{:?}", arr4);


    /* mutating arrays:  */

    let mut mut_arr = [4;4];
    mut_arr[3] = 34;

    println!("{:?}", mut_arr);

    for i in 0..mut_arr.len(){
        println!("{}", mut_arr[i]);
    }


    /* Rust slice: used to access memory portions of data types such as arrays, vectors, etc.. */

    let mut slice = &mut_arr[2..4]; 
    
    println!("{:?}", slice);
    
    /* one can omit the start and end indices like so.. */

    println!("{:?}, {:?}", &mut_arr[..3], &mut_arr[2..]);

    /* changing the elements at a slice */
    let mut arr = [6; 7];

    let mutable_slice = & mut arr[..2];
    mutable_slice[0] = 21;
    println!("{:?}", arr);

    /* string slicing:  */
    let my_string = String::from("This is amazing!!");

    let string_slice = &my_string[0..my_string.len() - 2];
    println!("{:?}", string_slice);

}