/*!
Minor utilities.
*/

/**
 * Returns the default value of a type.
 */
pub fn default<T: Default>() -> T {
    T::default()
}