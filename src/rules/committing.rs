use clap::ArgMatches;

use crate::rules::rule::Rule;

pub const COMMITTING_HEADING: &str = "## Committing\n\n";

pub fn committing<'a>(arguments: &'a ArgMatches) -> Vec<Rule<'a>> {
    vec![
        Rule {
            flag: arguments.occurrences_of("small-commits") > 0,
            rule: "- [Small Commits][small-commits]\n",
            link: "[small-commits]: <https://github.com/sean-hut/contributing-rules/blob/develop/rules/committing/small-commits.md>\n",
        verbose: "Included small commits rule",
        },
        Rule {
            flag: arguments.occurrences_of("sign-commit") > 0,
            rule: "- [Sign Your Commits][sign-commits]\n",
            link: "[sign-commits]: <https://github.com/sean-hut/contributing-rules/blob/develop/rules/committing/sign-and-signoff-commits.md>\n",
            verbose: "Included sign commits rule",
        },
        Rule {
            flag: arguments.occurrences_of("amaranth") > 0,
            rule: "- [Amaranth Commit Message Format][amaranth]\n",
            link: "[amaranth]: <https://github.com/sean-hut/contributing-rules/blob/develop/rules/committing/commit-message-format/amaranth-commit-message-format.md>\n",
            verbose: "Included Amaranth commit message format rule",
        },
        Rule {
            flag: arguments.occurrences_of("tpope") > 0,
            rule: "- [tpope Commit Message Format][tpope]\n",
            link: "[tpope]: <https://github.com/sean-hut/contributing-rules/blob/develop/rules/committing/commit-message-format/tpope-commit-message-format.md>\n",
            verbose: "Included tpope commit message format rule",
        },
    ]
}

pub fn any_committing(arguments: &ArgMatches) -> bool {
    committing(&arguments).iter().map(|x| x.flag).any(|x| x)
}
