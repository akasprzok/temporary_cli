use clap::{Command};

fn cli() -> Command {
    Command::new("temporary")
        .about("All about Andreas")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("projects")
                .about("lists projects I've been working on")
        )
        .subcommand(
            Command::new("resume")
            .about("get my resume")
        )
        .subcommand(
            Command::new("blog")
            .about("interact with my blog")
            .arg_required_else_help(true)
            .subcommand(
                Command::new("latest")
                .about("get the latest blog post")
            )
            .subcommand(Command::new("tags")
                .about("get possible tags for blog")
        )
        )
}

fn main() {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("projects", _sub_matches)) => {
            println!(
                "listing projects!"
            );
        }
        Some(("resume", _sub_matches)) => {
            println!("getting resume!")
        }
        Some(("blog", sub_matches)) => {
            let blog_command = sub_matches.subcommand().unwrap_or(("latest", sub_matches));
            match blog_command {
                ("latest", _sub_matches) => {
                    println!("getting latest blog post!")
                }
                ("tags", _sub_matches) => {
                    println!("getting tags!")
                }
                (command, _) => {
                    unreachable!("Unsupported subcommand `{}`", command)
                }
            }
        }
        _ => unreachable!(),
    }
}
