
pub trait Revar {
    fn revar(self, _: &[char]) -> String;
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

#[test]
fn unvar_revar() {
    assert_eq!("(i+j)/k*j", "(b+a)/c*a".revar(&['j', 'i', 'k']));
    assert_eq!("(b+a)/c*a", "(i+j)/k*j".unvar(&['j', 'i', 'k']));
}

