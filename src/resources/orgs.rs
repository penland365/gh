// GitHub OrgSummary. See https://developer.github.com/v3/orgs/
// A base GitHub User
#[derive(RustcEncodable, RustcDecodable, Serialize, Deserialize)]
pub struct OrgSummary {
    pub login: String,
    pub id: u32,
    pub url: String,
    pub repos_url: String,
    pub events_url: String,
    pub hooks_url: String,
    pub issues_url: String,
    pub members_url: String,
    pub public_members_url: String,
    pub avatar_url: String,
    pub description: String
}
