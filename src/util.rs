use clap::App;
use {GithubError, GithubResult};
use std::str;

// Write the the Clap::App help String
pub fn build_app_help_string(app: &App) -> String {
    let mut vec: Vec<u8> = Vec::new();
    app.write_help(&mut vec).ok().expect("Failed to write to Vector<u8>");
    return match str::from_utf8(&vec) {
        Ok(s)  => s.to_owned(),
        Err(e) => panic!("Failure to convert Vector<u8> to utf8 string: {}", e)
    }
}

#[cfg(test)]
mod tests {
use clap::App;

    #[test]
    fn test_build_app_help_string() {
        let expected = r#"gh 

USAGE:
    gh

"#;
        let result   = super::build_app_help_string(&App::new("gh"));
        assert_eq!(expected, result);
    }

}
