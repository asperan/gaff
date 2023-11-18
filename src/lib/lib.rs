use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    git_uri: String,

    #[arg(short, long, help = "The git refspec to clone. If not present, the default branch is used.")]
    refspec: Option<String>,

    #[arg(long, help = "Skip the automap step: there will be no attempt to automatically map script with a hook name to that hook")]
    skip_automap: bool,

    #[arg(long, help = "The directory in the repository from which start to search for scripts")]
    hooks_dir: Option<String>,
}

pub fn entrypoint() {
    let cli = Cli::parse();
    dbg!(&cli);

    // TODO: clone repository (using reference, if given)
    // TODO: find all shell scripts
    // TODO: ask the user which of them to consider
    // TODO: automap scripts with a hook name to that hook
    // TODO: for non-automapped scripts, ask the user how they want to map them
    // TODO: for each selected script copy (and rename, if needed) it in ./.git/hooks
    println!("Hello, world!");
}

