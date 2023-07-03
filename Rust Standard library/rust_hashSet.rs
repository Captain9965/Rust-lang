use std::collections::HashSet;

// it allows us to store values without duplicates:
fn main(){
    let mut set = HashSet::new();
    println!("{:?}", set);

    // Adding values:
    set.insert("Goodman");
    // is ignored: 
    set.insert("Goodman");
    set.insert("Alisha");

    println!("{:?}", set);

    // remove elements:
    set.remove("Alisha");

    println!("{:?}", set);

    // iteraring over the hashSet:

    for item in set{
        println!("{}", item);
    }

    // creating a set using from() method:

    let set2 = HashSet::from([2, 4, 5, 6, 6]);
    let set3 = HashSet::from([3, 4, 8, 98, 4]);

    // union between the sets. The union method returns an iterator hence, we use collections to get an actual result:
    let union_result : HashSet<_> = set2.union(&set3).collect();
    let intersection_result : HashSet <_> = set2.intersection(&set3).collect();
    let difference_result : HashSet<_> = set3.difference(&set2).collect();
    let symmetric_result : HashSet <_> = set3.symmetric_difference(&set2).collect();

    println!("Union: {:?} \nIntersection: {:?} \nDifference : {:?}\nSymmetric difference : {:?}", union_result, intersection_result, difference_result, symmetric_result);
}