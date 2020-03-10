#[macro_use]
extern crate log;
extern crate pretty_env_logger;
#[macro_use]
extern crate plugins;
extern crate hubcaps;
extern crate tokio;
use futures::Stream;
use hubcaps::issues::{IssueListOptions, State};
use hubcaps::{Credentials, Github};
use std::env::var;
use tokio::runtime::Runtime;

use plugins::gherkin::Feature;
use plugins::{Arg, PluginResult};
use std::collections::HashMap;

fn run(args: HashMap<String, String>) -> Option<PluginResult> {
    let token = args
        .get("TOKEN")
        .map(|s| s.to_string())
        .or_else(|| var("GITHUB_TOKEN").ok())
        .unwrap();
    let repo = args.get("REPO").unwrap();
    let number = args
        .get("number")
        .and_then(|number| number.parse::<u64>().ok());
    let github = Github::new(String::from("Bat-nets"), Credentials::Token(token));
    let parts: Vec<&str> = repo.split('/').collect();
    let mut rt = Runtime::new().unwrap();
    match rt.block_on(
        github
            .repo(parts[0], parts[1])
            .issues()
            .iter(
                &IssueListOptions::builder()
                    .per_page(100)
                    .state(State::Open)
                    .build(),
            )
            .filter_map(move |issue| {
                let issue_number = issue.number;
                let title = issue.title;
                let body = issue.body.map(|body| (issue_number, title, body));
                match number {
                    None => body,
                    Some(number) if number == issue_number => body,
                    _ => None,
                }
            })
            .for_each(move |(number, title, body)| {
                info!("{}: {} ({})", number, title, body);
                Ok(())
            }),
    ) {
        Err(err) => {
            error!("{}", err);
            None
        }
        Ok(_) => Some(PluginResult::new(
            "github.feature",
            Feature::new("test".to_string()),
        )),
    }
}

on_plugin_load!({
    pretty_env_logger::init();
    debug!("🕸️ Bat-nets");
});

on_plugin_unload!({
    debug!("🕸️ Bat-nets");
});

#[allow(clippy::not_unsafe_ptr_arg_deref)]
declare_plugin!(
    vec![
        Arg::new("REPO", "<REPO> 'Github repo'"),
        Arg::new(
            "TOKEN",
            "[TOKEN] 'Github personal token. Override the GITHUB_TOKEN env variable'"
        ),
        Arg::new("number", "-n, --number=[number] 'Issue number'")
    ],
    run
);
