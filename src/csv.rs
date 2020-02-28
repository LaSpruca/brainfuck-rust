pub fn parse(input: String) -> Vec<i32> {
    let mut yes: Vec<i32> =  vec!(1);
    yes.pop();
    let mut ok:Vec<&str> = input.split(",").collect();
    for y in ok {
        let o = String::from(y);
        yes.push(o.parse::<i32>().unwrap());
    }
    yes
}

pub struct Input {
    values: Vec<i32>,
    pointer: i32
}

impl crate::brainfuck::Next for Input {
    fn next(&mut self) -> i32 {
        let ret: i32 = self.values[self.pointer];
        self.pointer += 1;
        ret
    }
}