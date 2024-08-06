pub(crate) fn demo_variables() {
    let mut x = 5; // without keyword mut, throws error at line 6 because it is immutable
    println!("The value stored is: {x}");
    
    // set differet value
    x = 6; 
    println!("The value stored is: {x}");
}