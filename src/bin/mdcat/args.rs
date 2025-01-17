// Copyright 2018-2020 Sebastian Wiesner <sebastian@swsnr.de>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use clap::*;

pub(crate) fn app(default_columns: &str) -> App<'_> {
    app_from_crate!()
        .setting(AppSettings::DontCollapseArgsInUsage)
        .setting(AppSettings::DeriveDisplayOrder)
        .term_width(80)
        .after_help(
            "\
See 'man 1 mdcat' for more information.

Report issues to <https://github.com/lunaryorn/mdcat>.",
        )
        .arg(
            Arg::new("paginate")
                .short('p')
                .long("--paginate")
                .help("Paginate the output of mdcat with a pager like less."),
        )
        .arg(
            Arg::new("no_pager")
                .short('P')
                .long("--no-pager")
                .help("Do not page output.  Default if invoked as mdcat")
                .overrides_with("paginate"),
        )
        .arg(
            Arg::new("filenames")
                .multiple_occurrences(true)
                .help("The file to read.  If - read from standard input instead")
                .default_value("-"),
        )
        .arg(
            Arg::new("no_colour")
                .short('c')
                .long("--no-colour")
                .aliases(&["nocolour", "no-color", "nocolor"])
                .help("Disable all colours and other styles."),
        )
        .arg(
            Arg::new("columns")
                .long("columns")
                .help("Maximum number of columns to use for output")
                .default_value(default_columns),
        )
        .arg(
            Arg::new("local_only")
                .short('l')
                .long("local")
                .help("Do not load remote resources like images"),
        )
        .arg(
            Arg::new("dump_events")
                .long("dump-events")
                .help("Dump Markdown parser events and exit")
                .hide(true),
        )
        .arg(
            Arg::new("fail_fast")
                .long("fail")
                .help("Exit immediately if any error occurs processing an input file"),
        )
        .arg(
            Arg::new("detect_only")
                .long("detect-only")
                .help("Only detect the terminal type and exit")
                .hide(true),
        )
        .arg(
            Arg::new("ansi_only")
                .long("ansi-only")
                .help("Limit to standard ANSI formatting")
                .conflicts_with("no_colour")
                .hide(true),
        )
}

#[test]
fn verify_app() {
    app("80").debug_assert();
}
