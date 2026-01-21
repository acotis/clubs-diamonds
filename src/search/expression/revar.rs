
pub trait Revar {
    fn revar(self, _: &[char]) -> String;
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
}

