
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();

    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for (i, item) in matrix.iter_mut().enumerate().take(len1 + 1) {
        item[0] = i;
    }

    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    for (i, c1) in s1.chars().enumerate() {
        for (j, c2) in s2.chars().enumerate() {
            let cost = if c1 == c2 { 0 } else { 1 };

            matrix[i + 1][j + 1] = (matrix[i][j + 1] + 1)
                .min(matrix[i + 1][j] + 1)
                .min(matrix[i][j] + cost);
        }
    }

    matrix[len1][len2]
}


/// Find the most similar name in a list of names to a target name
///
/// # Arguments
///
/// * `names` - A list of names to search
/// * `target` - The target name
///
/// # Returns
///
/// The most similar name in `names` to `target`
///
/// # Example
///
/// ```
/// use popper_semantic_analyzer::tool::name_similarity::find_similar_name;
///
/// let names = vec![
///    String::from("foo"),
///    String::from("bar"),
///    String::from("baz"),
/// ];
///
/// let target = "f00";
///
/// let result = find_similar_name(&names, target);
///
/// assert_eq!(result, Some(&String::from("foo")));
/// ```
///
pub fn find_similar_name<'a>(names: &'a [String], target: &'a str) -> Option<&'a String> {
    let mut min_distance = usize::MAX;
    let mut closest_name = None;

    for name in names {
        let distance = levenshtein_distance(name, target);

        if distance < min_distance {
            min_distance = distance;
            closest_name = Some(name);
        }
    }

    closest_name
}
