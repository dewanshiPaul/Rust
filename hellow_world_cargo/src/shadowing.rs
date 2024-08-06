pub(crate) fn demo_shadow() {
    let x = "   6";
    {
        let x = x.len();
        println!("The value of x is {x}");
    }

    println!("The value of x is {x}");
}