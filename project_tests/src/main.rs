use std::io::{BufReader, BufRead};

pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    _read_stdin(&mut reader)
}

fn _read_stdin<R: BufRead>(reader: &mut R) -> String {
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("Failed to read input line");
    line.trim().to_string()
}

#[cfg(test)]
mod tests {

    use super::_read_stdin;
    use std::io::Cursor;

    #[test] 
    fn test_read_input() {
        let input = "Hello World!\n";
        let expected_output = "Hello World!";
        let mut reader = Cursor::new(input);
        let output =  _read_stdin(&mut reader);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_read_empty_input() {
        let input = "";
        let expected_output = "";
        let mut reader = Cursor::new(input);
        let output = _read_stdin(&mut reader);
        assert_eq!(output, expected_output);
    }
}
