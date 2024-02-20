use clap::Command;

pub fn cli() -> Command {
    Command::new("timy").subcommand(
        Command::new("the_LaTeX-syntax_really-is_ergonomic")
            .about("Print the entries in the lesser LaTeX format")
            .long_flag("yes-i_really-want_it-in_the-LaTeX_format%")
            .arg_required_else_help(false),
    )
}
