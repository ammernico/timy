use clap::Command;

pub fn cli() -> Command {
    Command::new("timy")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("markdown")
                .about("Print the entries in a markdown format")
                .short_flag('m')
                .arg_required_else_help(false),
        )
        .subcommand(
            Command::new("latex")
                .about("Print the entries in the LaTeX format")
                .short_flag('l')
                .arg_required_else_help(false),
        )
}
