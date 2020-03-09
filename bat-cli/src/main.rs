#[macro_use]
extern crate clap;
extern crate plugins;
extern crate term;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

mod batcave;
use clap::{App, SubCommand};
use plugins::PluginManager;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    pretty_env_logger::init();
    let mut pm = PluginManager::new();

    if let Err(err) = (&mut pm).load_plugins(get_lib_dir()) {
        panic!("{}", err)
    }

    let yaml = load_yaml!("cli.yml");
    let mut app = pm.list_commands().iter().fold(
        App::from_yaml(yaml),
        |acc, (name, version, command, args)| {
            acc.subcommand(
                SubCommand::with_name(*command)
                    .about(*name)
                    .version(*version)
                    .args_from_usage(*args),
            )
        },
    );

    let matches = app.clone().get_matches();
    match matches.subcommand() {
        ("man", _) => batman(),
        ("cave", Some(subcommand)) => batcave::command(subcommand, &pm),
        (name, Some(subcommand)) => match pm.get_command(|plugin_name| plugin_name == name) {
            None => (&mut app).print_long_help().unwrap(),
            Some(plugin) => {
                trace!("Executing: {}", plugin.name());
                let output = subcommand.value_of("output").unwrap_or("out");
                let params: HashMap<String, String> = plugin
                    .args_name()
                    .iter()
                    .filter_map(|name| {
                        subcommand
                            .value_of(name)
                            .map(|p| ((*name).to_string(), p.to_string()))
                    })
                    .collect();
                match plugin.run(params) {
                    Some(feature) => {
                        fs::create_dir_all(&output).unwrap();
                        let path = Path::new(&output).join(feature.filename());
                        let mut file = File::create(path).unwrap();
                        write!(file, "{}", feature.feature()).unwrap();
                    }
                    None => std::process::exit(1),
                }
            }
        },
        _ => (&mut app).print_long_help().unwrap(),
    }
}

fn batman() {
    let mut t = term::stdout().unwrap();
    write!(t, "🦇 ").unwrap();
    t.fg(term::color::YELLOW).unwrap();
    write!(t, "I'm not Bruce Wayne").unwrap();
    t.reset().unwrap();
    writeln!(t, " 🦇").unwrap();
}

fn get_lib_dir() -> String {
    env::var("BAT_LIB").unwrap_or_else(|_| ".".to_string())
}
