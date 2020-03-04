pub fn parse(input: String) -> Vec<i32> {
    let mut yes: Vec<i32> =  Vec::new();
    let ok:Vec<&str> = input.split(",").collect();
    for y in ok {
        let o = String::from(y);
        yes.push(o.parse::<i32>().unwrap());
    }
    yes
}
