
// Iterators allow us to create a sequence of values that allow us to iterate over each value of the sequence:
fn main(){
    let mut numbers = [4, 5, 6, 6, 6];

    for num in numbers.iter(){
        println!("{}", num);
    }

    let vector = vec![3, 4, 5, 6];
    let mut iterator = vector.iter();

    // Next either returns Some value in Options collection or None:
    println!("{:?}", iterator.next()); // 3
    println!("{:?}", iterator.next()); // 4

    /************************ Different ways of creating an iterator: ***********************/
    /* 
        With the iter() method, the value is borrowed (referenced), and hence it is available for use after as  seen above
        With into_iter(), the value is moved, hence unavailable:
        With iter_mut(), we can mutably borrow each element and modify the collection: 

    */

    for num in numbers.into_iter(){
        println!("{}", num);
    }

    println!("{:?}", numbers);

    for num in numbers.iter_mut(){
        println!("{}", num);
        // must use the dereferencing operator to dereference mutably borrowed value: 
        
        *num = 0;
    }

    println!("{:?}", numbers);
}