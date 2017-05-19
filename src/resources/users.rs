// GitHub users. See https://developer.github.com/v3/users/
// A base GitHub User
#[derive(Serialize, Deserialize)]
pub struct User {
    pub login: String,
    pub id: u32,
    pub location: String,
    pub bio: Option<String>
}

fn reverse<T: Clone>(xs: &[T]) -> Vec<T> {
    let mut rev = vec!();
    for x in xs.iter() {
        rev.insert(0, x.clone())
    }
    rev
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }

    use super::reverse;
	quickcheck! {
        fn prop_test(xs: Vec<u32>) -> bool {
            xs == reverse(&reverse(&xs))
        }
	}
}
