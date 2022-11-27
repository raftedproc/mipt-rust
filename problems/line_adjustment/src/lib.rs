use std::vec::Vec;

#[derive(Debug)]
enum PushWordsError {
    Overflow,
}

#[derive(Debug)]
struct WordsWithSpacesIndices<'a> {
    words_: Vec<&'a str>,
    len_: usize,
    is_last_: bool,
}

impl<'a> WordsWithSpacesIndices<'a> {
    pub fn push_word(&mut self, word: &'a str, line_width: usize) -> Result<(), PushWordsError> {
        if self.len_ + word.len() + 1 > line_width {
            return Err(PushWordsError::Overflow);
        }
        self.len_ += word.len();
        self.words_.push(word);
        Ok(())
    }
    pub fn to_string(&mut self, line_width: usize) -> Result<String, ()> {
        assert!(self.len_ <= line_width);
        let mut s = String::with_capacity(line_width);
        let spaces_to_add = line_width - self.len_;
        let (spaces_per_word, mut additional_spaces) = match self.words_.len() {
            0 => {
                return Ok("".to_string());
            }
            1 => (spaces_to_add, 0),
            v => (spaces_to_add / (v - 1), spaces_to_add % (v - 1)),
        };
        let spaces_plus_1 = " ".repeat(spaces_per_word + 1);
        let spaces = &spaces_plus_1[0..spaces_plus_1.len() - 1];
        let mut word_pos = 0;
        s = self.words_.iter().fold(s, |s, &w| {
            let mut res = s;
            res.push_str(w);
            // 3 args logical function to choose how many spaces to add.
            res.push_str(
                match (
                    self.words_.len() == 1,
                    word_pos + 1 == self.words_.len(),
                    additional_spaces > 0,
                ) {
                    (true, _, _) => spaces,
                    (false, true, _) => "",
                    (false, false, false) => spaces,
                    (false, false, true) => {
                        additional_spaces -= 1;
                        spaces_plus_1.as_str()
                    }
                },
            );
            word_pos += 1;
            res
        });
        // The delimiter of aligned sentences.
        if !self.is_last_ {
            s.push('\n');
        }
        Ok(s)
    }
    pub fn set_last(&mut self) {
        self.is_last_ = true;
    }
}

// Unsafe позволит подхачить str, прибавив единицу к длине, чтобы не добавлятть пробелы в цикле.
pub fn transform(input: &str, line_width: usize) -> String {
    let mut v: Vec<&str> = input.split_whitespace().collect();
    let mut r: Vec<WordsWithSpacesIndices> = vec![WordsWithSpacesIndices {
        words_: vec![],
        len_: 0,
        is_last_: false,
    }];
    r = v.iter_mut().fold(r, |r, &mut v| {
        let mut res = r;
        let last = res.last_mut().expect("No elements in res");
        if let Err(_) = last.push_word(v, line_width) {
            res.push(WordsWithSpacesIndices {
                words_: vec![v],
                len_: v.len(),
                is_last_: false,
            });
        }
        res
    });
    r.last_mut()
        .expect("Cannot mark the last word b/c no words found in r")
        .set_last();
    // Here can be a better estimate using prev fold.
    let result = String::with_capacity(input.len() + v.len() * 2); //original input length + \n + at least one space
    let result = r.iter_mut().fold(result, |result, w| {
        let mut r = result;
        r.push_str(&w.to_string(line_width).expect("Some err").as_str());
        r
    });
    result
}

#[cfg(test)]
mod tests {
    use super::transform;

    #[test]
    fn simple() {
        let test_cases = [
            ("", 5, ""),
            ("test", 5, "test "),
            ("Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua", 12,
             "Lorem  ipsum\ndolor    sit\namet        \nconsectetur \nadipiscing  \nelit  sed do\neiusmod     \ntempor      \nincididunt  \nut labore et\ndolore magna\naliqua      "),
            ("Lorem     ipsum    dolor", 17, "Lorem ipsum dolor"),
        ];

        for &(input, line_width, expected) in &test_cases {
            println!("input: '{}'", input);
            assert_eq!(transform(input, line_width), expected);
        }
    }
}
