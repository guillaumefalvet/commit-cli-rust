use std::process;
use clap::{Arg, Command};
use owo_colors::OwoColorize;

// Import the git module
mod git;
use git::{parse_git_branch, commit};

// Main function
fn main() {
    // ANSI Regular
    const OWO: &str = r#"

    ██████╗ ██████╗ ███╗   ███╗███╗   ███╗██╗████████╗    ████████╗ ██████╗  ██████╗ ██╗
    ██╔════╝██╔═══██╗████╗ ████║████╗ ████║██║╚══██╔══╝    ╚══██╔══╝██╔═══██╗██╔═══██╗██║
    ██║     ██║   ██║██╔████╔██║██╔████╔██║██║   ██║          ██║   ██║   ██║██║   ██║██║
    ██║     ██║   ██║██║╚██╔╝██║██║╚██╔╝██║██║   ██║          ██║   ██║   ██║██║   ██║██║
    ╚██████╗╚██████╔╝██║ ╚═╝ ██║██║ ╚═╝ ██║██║   ██║          ██║   ╚██████╔╝╚██████╔╝███████╗
     ╚═════╝ ╚═════╝ ╚═╝     ╚═╝╚═╝     ╚═╝╚═╝   ╚═╝          ╚═╝    ╚═════╝  ╚═════╝ ╚══════╝
"#;
println!("\n\n\n\n\n{}", OWO.fg_rgb::<0x2E, 0x31, 0x92>().bold());
    // Parse command-line arguments using clap
    let matches = Command::new("commit-helper by Guillaume Falvet")
        .version("1.0.0")
        .about("A Git commit helper tool")
        .arg(
            Arg::new("type")
                .short('t')
                .long("type")
                .value_name("TYPE")
                .required(true)
                .help("The type of the commit. e.g., feat, fix, docs, style, refactor, test, chore")
        )
        .arg(
            Arg::new("scope")
                .short('s')
                .long("scope")
                .value_name("SCOPE")
                .help("The scope of the commit. e.g., component name")
        )
        .arg(
            Arg::new("message")
                .short('m')
                .long("message")
                .value_name("MESSAGE")
                .required(true)
                .help("The commit message")
        )
        .arg(
            Arg::new("dry-run")
                .short('d')
                .long("dry-run")
                .help("Perform a dry run without committing")
                .num_args(0)
                .required(false)
        )
        .get_matches();

    // Extract command-line arguments
    let type_arg = matches.get_one::<String>("type").unwrap();
    let scope_arg = matches.get_one::<String>("scope").map(|s| s.to_string());
    let message_arg = matches.get_one::<String>("message").unwrap();
    let dry_run = matches.get_one::<bool>("dry-run").unwrap_or(&false);
    // Get the current branch name
    let branch_name = match parse_git_branch() {
        Some(name) => name.trim().to_string(),
        None => {
            eprintln!("Not a Git repository or you are not on any branch.");
            process::exit(1);
        }
    };

    // Avoid commits on the develop branch
    if branch_name == "develop" {
        eprintln!("Commits to 'develop' branch are not allowed with this cli tool.");
        process::exit(1);
    }

    // Check if the branch name contains a slash and extract the part after it
    let jira_ticket = if let Some(index) = branch_name.find('/') {
        &branch_name[index + 1..]
    } else {
        eprintln!("This is not a git flow branch. Please use a feature or bugfix branch.");
        process::exit(1);
    };

    // Combine type and scope if scope is provided
    let concat = match &scope_arg {
        Some(scope) => format!("{} ({})", type_arg, scope),
        None => type_arg.clone()
    };

    // Format the commit message
    let commit_message = format!("{} {}: {}", jira_ticket, concat, message_arg);

    //Perform dry run or actual committing
    if dry_run.clone() {
        println!("Dry run enabled. The commit command would be:");
        println!("git commit -m \"{}\"", commit_message);
    } else {
        println!("git commit -m \"{}\"", commit_message);
        if let Some(output) = commit(commit_message) {
            println!("Commit output: {}", output);
        } else {
            println!("Failed to commit changes.");
        }
    }
}
