impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack: Vec<&str> = Vec::new();
        for dir in path.split("/") {
            match dir {
                "." | "" => (),
                ".." => {stack.pop();},
                x => {stack.push(x);}
            }
        }
        String::from("/") + &stack.join("/")
    }
}
