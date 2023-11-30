pub fn reverse(input: &str) -> String {
    let mut string_result = String::from("");
    let mut stack : Vec<char> = Vec::new();
    for character in input.chars() {
        stack.push(character);
    }
    while let Some(value) = stack.pop() {
        string_result = format!("{}{}",string_result,value.to_string());
    }
    string_result
}



#[cfg(test)]
mod test {
    use super::reverse;
    #[test]
    fn first_test() {
        assert_eq!(reverse("abd"),"dba".to_string());
    }
}