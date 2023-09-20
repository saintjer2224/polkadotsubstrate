fn concatenate_strings(string1: &str, string2: &str) -> String {
    let mut result = String::new();
    result.push_str(string1);
    result.push_str(string2);
    result
}

fn main() {
    let s1 = String::from("hello");
    let s2 = String::from(" world!");

    let concatenated_string = concatenate_strings(&s1,&s2);

    println!("{:?}", concatenated_string);
}
