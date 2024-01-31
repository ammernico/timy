use clap::Command;

pub fn cli() -> Command {
    Command::new("timy").subcommand_required(true).subcommand(
        Command::new("print")
            .short_flag('p')
            .subcommand_required(true)
            .subcommand(
                Command::new("markdown")
                    .about("Print the entries in a superior markdown table")
                    .short_flag('m')
                    .long_flag("markdown")
                    .arg_required_else_help(false),
            )
            .subcommand(
                Command::new("latex")
                    .about("Print the entries in the lesser LaTeX format")
                    .short_flag('l')
                    .long_flag("latex")
                    .arg_required_else_help(false),
            ),
    )
}
