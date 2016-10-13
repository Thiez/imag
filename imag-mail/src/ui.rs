use clap::{Arg, ArgGroup, App, SubCommand};

pub fn build_ui<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
    app
        .subcommand(SubCommand::with_name("import-mail")
                    .about("Import a mail (create a reference to it) (Maildir)")
                    .version("0.1")
                    .arg(Arg::with_name("path")
                         .long("path")
                         .short("p")
                         .takes_value(true)
                         .required(true)
                         .help("Path to the mail file or a directory which is then searched recursively")
                         .value_name("PATH"))
                    )

        .subcommand(SubCommand::with_name("list")
                    .about("List all stored references to mails")
                    .version("0.1")

                    // TODO: Thee following four arguments are the same as in imag-ref.
                    // We should make these importable from libimagref.

                    .arg(Arg::with_name("check-dead")
                         .long("check-dead")
                         .short("d")
                         .help("Check each reference whether it is dead"))

                    .arg(Arg::with_name("check-changed")
                         .long("check-changed")
                         .short("c")
                         .help("Check whether a reference had changed (content or permissions)"))

                    .arg(Arg::with_name("check-changed-content")
                         .long("check-changed-content")
                         .short("C")
                         .help("Check whether the content of the referenced file changed"))

                    .arg(Arg::with_name("check-changed-permissions")
                         .long("check-changed-perms")
                         .short("P")
                         .help("Check whether the permissions of the referenced file changed"))

                    )

        .subcommand(SubCommand::with_name("mail-store")
                    .about("Operations on (subsets of) all mails")
                    .version("0.1")
                    .subcommand(SubCommand::with_name("update-refs")
                                .about("Create references based on Message-IDs for all loaded mails")
                                .version("0.1"))
                    // TODO: We really should be able to filter here.
                    )
}

