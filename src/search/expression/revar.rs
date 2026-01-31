
/// Trait implemented on the `&str` type to imbue it with the `.revar()`
/// and `.unvar()` methods.

pub trait Revar {

    /// Rename the variables in a string from their default names ('a', 'b',
    /// 'c', etc) to any custom set of names.
    ///
    /// ```
    /// use clubs_diamonds::Revar;
    ///
    /// assert_eq!("(i+j)/k*j", "(b+a)/c*a".revar(&['j', 'i', 'k']));
    /// ```
    ///
    /// Useful to apply in sequence after formatting an
    /// [`Expression`][crate::Expression], since an
    /// [`Expression`][crate::Expression] always renders itself with
    /// default variable names.

    fn revar(self, _: &[char]) -> String;

    /// Un-name the variables in a string from a provided custom set of names
    /// back to their default names ('a', 'b', 'c', etc).
    ///
    /// ```
    /// use clubs_diamonds::Revar;
    ///
    /// assert_eq!("(b+a)/c*a", "(i+j)/k*j".unvar(&['j', 'i', 'k']));
    /// ```
    ///
    /// Useful to apply in sequence before parsing an
    /// [`Expression`][crate::Expression], since an
    /// [`Expression`][crate::Expression] always parses itself from a
    /// string assuming default variable names.

    fn unvar(self, _: &[char]) -> String;
}

impl Revar for &str {
    fn revar(self, new_names: &[char]) -> String {
        self.chars()
            .map(|c| match "abcdefghijklmnopqrstuvwxyz".find(c) {
                Some(index) => new_names[index],
                None => c,
            })
            .collect()
    }

    fn unvar(self, old_names: &[char]) -> String {
        self.chars()
            .map(|c| match old_names.iter().position(|x| *x == c) {
                Some(index) => "abcdefghijklmnopqrstuvwxyz".chars().nth(index).unwrap(),
                None => c,
            })
            .collect()
    }
}

