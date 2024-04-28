fn main() {
    let my_string = String::from("hello, world!");

    // immutable reference to my_string
    let my_ref = &my_string;

    println!("::: my_string: {}", my_string);
    println!("::: my_ref: {}", my_ref);

    print_string(&my_string);
    print_string(my_ref);

    // mutable reference
    let mut my_mutable_string = String::from("Goodbye ");

    print_string(&my_mutable_string);

    change_string(&mut my_mutable_string);

    print_string(&my_mutable_string);

    // change string using a reference
    let my_mutable_reference = &mut my_mutable_string;
    my_mutable_reference.push_str(" mad world!");


    println!("::: my_mutable_reference: {}", my_mutable_reference);
    println!("::: my_mutable_string: {}", my_mutable_string);

    let my_other_mutable_reference  = &my_mutable_string;
    println!("::: my_other_mutable_reference: {}", my_other_mutable_reference);
}

fn print_string(s: &String) {
    println!("{}", s);
}

fn change_string(s: &mut String) {
    return s.push_str(" world!");
}
