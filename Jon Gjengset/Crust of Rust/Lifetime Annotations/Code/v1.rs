// This means the compiler will emit a warning (but not an error) if the rule is violated.
#![warn(rust_2018_idioms)]  //  #![...] – Inner Attribute, Inner attributes apply to the entire crate (i.e. the whole file or module).


// The 'haystack annotation ensures that the StrSplit doesn’t try to borrow the string for longer than it exists.
 // Example where it would be a problem (if lifetimes weren't used correctly):
    // let splitter2;
    // {
    //     let temp_string = String::from("short-lived");
    //     splitter2 = StrSplit::new(&temp_string, "-"); // ERROR!  `temp_string` doesn't live long enough
    // }
    // println!("{:?}", splitter2); // `splitter2` would be a dangling reference here!  (Compiler prevents this)

#[derive(Debug)]
pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    delimiter: D,
}

impl<'haystack, D> StrSplit<'haystack, D> {
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}



// '?' called the try operator or the question mark operator. It is a postfix operator that unwraps Result<T, E> and Option<T> values.
impl<'haystack, D> Iterator for StrSplit<'haystack, D>
where
    D: Delimiter,
{
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;  // converts an &mut Option<T> into an Option<&mut T>. In other words, instead of consuming or moving the Option out, you get a mutable reference to its interior value 
        if let Some((delim_start, delim_end)) = self.delimiter.find_next(remainder) {
            let until_delimiter = &remainder[..delim_start]; // The until_delimiter variable is designed to hold the segment of the self.remainder string that comes before the next occurrence of the delimiter. This segment is precisely what the StrSplit iterator should yield as its current item.
            *remainder = &remainder[delim_end..]; // // ← mutating the field in-place
            Some(until_delimiter)
        } else {
            self.remainder.take()  // self.remainder is set to None |  consuming the last piece of data in an iterator and preparing it for termination
        }
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}


//  strings are UTF-8 encoded, so characters (of type char) can be one to four bytes long
impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices() // byte idx = 1, char = 'é'
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + self.len_utf8()))
    }
}

pub fn until_char(s: &str, c: char) -> &'_ str {
    StrSplit::new(s, c)
        .next()
        .expect("StrSplit always gives at least one result")
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("hello world", 'o'), "hell");
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect(); // _ in a type is Rust’s way of saying “infer this type for me”
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn tail() {
    let haystack = "a b c d ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}