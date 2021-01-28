//TODO: Incorporate this into a derive

///Returns whether or not the given value is part of a given collection
///
/// # Arguments
///
/// * `value` - `T` to find
/// * `collection` - `Vec<T>` to search
///
/// # Example
/// ```
/// use crate::validation::is_in_collection;
///
/// assert_eq!(true, is_in_collection("SQL", vec!["SQL", "MongoDB", "Paper"]));
/// assert_eq!(false, is_in_collection("NoSQL", vec!["SQL", "MongoDB", "Paper"]));
///
/// assert_eq!(true, is_in_collection(32, vec![32, 44, 55]));
/// assert_eq!(false, is_in_collection(42, vec![32, 44, 55]));
/// ```
pub fn is_in_collection<T>(value: T, collection: Vec<T>) -> bool
where
    T: PartialEq + PartialOrd,
{
    for item in collection {
        if item.eq(&value) {
            return true;
        }
    }
    false
}
