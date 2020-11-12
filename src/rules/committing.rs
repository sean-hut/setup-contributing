use clap::ArgMatches;

use crate::rules::rule::Rule;

pub const COMMITTING_HEADING: &str = "## Committing\n\n";

pub fn committing<'a>(arguments: &'a ArgMatches) -> Vec<Rule<'a>> {
    vec![
        Rule {
            flag: arguments.is_present("small-commits"),
            rule: "- [Small Commits][small-commits]\n",
            link: "[small-commits]: <https://github.com/sean-hut/contributing-rules/blob/develop/rules/committing/small-commits.md>\n",
        verbose: "[Info] Included small commits rule",
        },
        Rule {
            flag: arguments.is_present("sign-commit"),
            rule: "- [Sign Your Commits][sign-commits]\n",
            link: "[sign-commits]: <https://github.com/sean-hut/contributing-rules/blob/develop/rules/committing/sign-and-signoff-commits.md>\n",
            verbose: "[Info] Included sign commits rule",
        },
        Rule {
            flag: arguments.is_present("amaranth"),
            rule: "- [Amaranth Commit Message Format][amaranth]\n",
            link: "[amaranth]: <https://github.com/sean-hut/contributing-rules/blob/develop/rules/committing/commit-message-format/amaranth-commit-message-format.md>\n",
            verbose: "[Info] Included Amaranth commit message format rule",
        },
        Rule {
            flag: arguments.is_present("tpope"),
            rule: "- [tpope Commit Message Format][tpope]\n",
            link: "[tpope]: <https://github.com/sean-hut/contributing-rules/blob/develop/rules/committing/commit-message-format/tpope-commit-message-format.md>\n",
            verbose: "[Info] Included tpope commit message format rule",
        },
    ]
}

pub fn any_committing(arguments: &ArgMatches) -> bool {
    committing(&arguments).iter().map(|x| x.flag).any(|x| x)
}
