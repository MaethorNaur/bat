use clap::ArgMatches;
use plugins::PluginManager;
use std::io::prelude::*;

pub fn command<'a>(subcommand: &ArgMatches<'a>, pm: &PluginManager) {
    let mut t = term::stdout().unwrap();
    write!(t, "🦇 ").unwrap();
    t.fg(term::color::YELLOW).unwrap();
    write!(t, "Welcome to the Batcave").unwrap();
    t.reset().unwrap();
    writeln!(t, " 🦇").unwrap();
    match subcommand.subcommand() {
        ("list", _) => list(pm),
        _ => writeln!(t, "{}", subcommand.usage()).unwrap(),
    }
}

fn list(pm: &PluginManager) {
    let mut t = term::stdout().unwrap();
    writeln!(t, "Available items:").unwrap();
    pm.list_commands()
        .iter()
        .for_each(|(name, version, command, _)| {
            write!(t, "\t").unwrap();
            t.fg(term::color::GREEN).unwrap();
            write!(t, "{}", name).unwrap();
            t.reset().unwrap();
            write!(t, " (").unwrap();
            t.attr(term::Attr::Bold).unwrap();
            t.fg(term::color::CYAN).unwrap();
            write!(t, "{}", command).unwrap();
            t.reset().unwrap();
            writeln!(t, "): {}", version).unwrap()
        })
}
