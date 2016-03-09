fn parse<S: ?Sized>(input: &S) -> Result<(), &'static str> 
    where S: AsRef<str>
{
    unimplemented!()
}

mod test {
    use super::*;

    use std::error::Error;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    macro_rules! read_file {
        ( $file_name:expr ) => {{
            let path = Path::new($file_name);
            let display = path.display();

            let mut file = match File::open(&path) {
                // The `description` method of `io::Error` returns a string that
                // describes the error
                Err(why) => panic!(
                    "couldn't open {}: {}",
                    display,
                    Error::description(&why)
                ),
                Ok(file) => file,
            };
            let mut s = String::new();
            match file.read_to_string(&mut s) {
                Err(why) => panic!(
                    "couldn't read {}: {}",
                    display,
                    Error::description(&why)
                ),
                Ok(_) => s,
            }
        }};
    }

    #[test]
    fn test_example1() {
        let input = read_file!("tests/input1.jade");
        let expected = read_file!("tests/expected1.html");
        assert!(parse(input).render() == expected);
    }
}
