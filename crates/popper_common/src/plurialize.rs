

///
/// this function takes a word and plurialize if there are many (>1) otherwise do nothing to the word
/// # Arguments
///
/// * `word`: the word to plurialize
/// * `number`: the number of this word
///
/// returns: the word as `String`
///
/// # Examples
///
/// ```rust
/// use popper_common::plurialize::plurialize;
/// let word = "cat";
/// let number_of_cat = 3;
/// assert_eq!(plurialize(word, number_of_cat), "cats");
/// let word = "dog";
/// let number_of_dog = 1;
/// assert_eq!(plurialize(word, number_of_dog), "dog")
/// ```
pub fn plurialize<T>(word: &str, number: T) -> String
where T: PartialOrd<usize> {
    format!(
        "{}{}", word,
        if number > 1 { "s" } else { "" }
    )
}

