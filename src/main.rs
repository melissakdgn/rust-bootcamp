fn concatenate_strings(s1: &str, s2: &str) -> String {
    let mut string1 = String::from(s1);
    string1.push_str(s2);
    let result = string1;
    result
}

fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from(" Patika"); 
    
    let concatenate_string = concatenate_strings(s1.as_str(), s2.as_str());
    println!("{}", concatenate_string);

}
