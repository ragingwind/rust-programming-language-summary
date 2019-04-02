#[allow(unused_variables)]
fn vector_basic() {
        // creating a new vector
    let v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];
    
    // updating a vector
    let mut v3: Vec<i32> = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    // dropping a vector drops its elements
    {
        let v = vec![1, 2, 3, 4];
    } // v goes out of scope and is freed here

    // reading element of vectors
    let third: &i32 = &v2[2];
    println!("{}", third);
    println!("{}", &v3[0]);

    match v2.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element")
    }
}

#[allow(unused_variables)]
fn vector_iterating() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i)
    }

    let mut v = vec![100, 32, 57];
    for i in & mut v {
        *i += 50;
    }

    // will print with mutated value with increasing by 50
    for i in &v {
        println!("{}", i)
    }
}

fn main() {
    vector_basic();
    vector_iterating();
}
