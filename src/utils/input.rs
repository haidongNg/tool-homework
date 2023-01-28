use std::io::stdin;
// Input ketboard
pub fn input(lable: &str) -> String {
    println!("{}>>>", lable);
    let mut val = String::new();
    stdin().read_line(&mut val).expect("Input error!");
    val
}