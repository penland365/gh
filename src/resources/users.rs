// GitHub users. See https://developer.github.com/v3/users/
// A base GitHub User
#[derive(Serialize, Deserialize)]
pub struct User {
    pub login: String,
    pub id: u32,
    pub location: String,
    pub bio: Option<String>
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(false);
    }
}
