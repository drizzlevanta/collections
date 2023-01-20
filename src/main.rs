fn main() {
    let mut v = Vec::new();
    v.push(5);
    v.push(7);
    println!("v is {:?}", v);

    let second = &v[1];
    let second2 = v.get(1);
    //v.push(9); //this line wouldn't compile because of borrowing rules
    println!("second is {second}");
    println!("second2 is {:?}", second2);

    v.push(9);

    for i in &mut v {
        *i += 10; //de-reference to modify value
    }

    for i in &v {
        println!("element is {i}");
    }

    crate::enum_vec::init();
}

mod enum_vec;
