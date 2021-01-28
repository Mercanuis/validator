/// Returns whether or not the `Option` is `Some`
///
/// # Arguments
///
/// * `val` - `Option` to be considered
///
/// # Example
/// ```
/// use crate::validation::is_not_null;
/// use uuid::Uuid;
///
/// let no_int: Option<i32> = None;
/// let some_str = Some("SQL".to_string());
/// let some_uuid = Some(Uuid::new_v4());
///
/// assert_eq!(true, is_not_null(&some_str));
/// assert_eq!(true, is_not_null(&some_uuid));
/// assert_eq!(false, is_not_null(&no_int));
/// ```
pub fn is_not_null<T>(val: &Option<T>) -> bool {
    val.is_some()
}
