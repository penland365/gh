use clap::{App, Arg, ArgMatches, SubCommand};

#[allow(dead_code)]
pub fn sub_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("pullreq")
                .about("List, Get, Create, Edit, and Merge a GitHub Pull Request.")
                .version(version!())
                .author("penland365 <Jeffrey.N.Davis@gmail.com>")
                .subcommand(SubCommand::with_name("list")
                                        .about("Lists GitHub Pull Requests.")
                                        .arg(Arg::with_name("owner")
                                                  .required(true)
                                                  .help("The GitHub owner of the repository."))
                                        .arg(Arg::with_name("repo")
                                                  .required(true)
                                                  .help("The GitHub repository."))
                                        .arg(Arg::with_name("format")
                                             .short("f")
                                             .long("format")
                                             .help("Sets the output format.")
                                             .value_name("json")
                                             .takes_value(true)))
                .subcommand(SubCommand::with_name("create")
                                        .about("Create a new GitHub Pull Request.")
                                        .arg(Arg::with_name("owner")
                                                  .required(true)
                                                  .help("The GitHub owner of the repository."))
                                        .arg(Arg::with_name("repo")
                                                  .required(true)
                                                  .help("The GitHub repository."))
                                        .arg(Arg::with_name("title")
                                                  .required(true)
                                                  .help("The title of the pull request."))
                                        .arg(Arg::with_name("head")
                                                  .required(true)
                                                  .help("The name of the branch where your changes are implemented. For cross-repository pull requests in the same network, namespace head with a user like this: username:branch."))
                                        .arg(Arg::with_name("base")
                                                  .required(true)
                                                  .help("The name of the branch you want the changes pulled into. This should be an existing branch on the current repository. You cannot submit a pull request to one repository that requests a merge to a base of another repository.")))

}

#[allow(dead_code)]
pub fn handle(matches: &ArgMatches) -> () {
    match matches.subcommand() {
        //("list", Some(list_matches)) => list_pull_reqs(list_matches) ,
        ("list", Some(_)) => list_pull_reqs() ,
        //("create", Some(create_matches)) => create_pull_request(create_matches),
        ("create", Some(_)) => create_pull_request(),
        ("", None)                   => println!("No subcommand was used for orgs"),
        (_, _)                       => unreachable!()
    }
}

#[allow(dead_code)]
fn list_pull_reqs() -> () {
//fn list_pull_reqs(matches: &ArgMatches) -> () {
   //let owner = matches.value_of("owner").unwrap().to_string();
   //let repo = matches.value_of("repo").unwrap().to_string();
   //let url = "https://api.github.com/repos/".to_string() +
   //     &owner +
   //     "/" +
   //     &repo +
   //     "/pulls";
   //let config = config::load_config();
   //let response = GitHub::get(&config.access_token, &url);
   // let st = GitHub::parse_json(&response);
   // println!("{}", st);
}

#[derive(Serialize, Deserialize)]
struct NewPullRequest {
    pub title: String,
    pub head: String,
    pub base: String
}

//fn create_pull_request(matches: &ArgMatches) -> () {
#[allow(dead_code)]
fn create_pull_request() -> () {
   //let owner = matches.value_of("owner").unwrap().to_string();
   //let repo = matches.value_of("repo").unwrap().to_string();
   //let url = "https://api.github.com/repos/".to_string() +
   //     &owner +
   //     "/" +
   //     &repo +
   //     "/pulls";
   //let pull_req = NewPullRequest {
   //     title: matches.value_of("title").unwrap().to_string(),
   //     head: matches.value_of("head").unwrap().to_string(),
   //     base: matches.value_of("base").unwrap().to_string()
   // };
   //let json_str = match serde_json::to_string(&pull_req) {
   //    Ok(x) => x,
   //    Err(_) => panic!("Error seriliazing pull request")
   // };
   //let bytes = json_str.as_bytes();
   //let config = config::load_config();
   //let response = GitHub::post(&config.access_token, &url, bytes);
   // let st = GitHub::parse_json(&response);
   // println!("{}", st);
}
