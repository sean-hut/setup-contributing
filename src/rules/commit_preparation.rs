use clap::ArgMatches;

use crate::rules::rule::Rule;

pub const PREPARATION_HEADING: &str = "## Preparation for Commits\n\n";

pub fn preparation<'a>(arguments: &'a ArgMatches) -> Vec<Rule<'a>> {
    vec![
        Rule {
            flag: arguments.occurrences_of("git-flow") > 0,
            rule: "- [git-flow Branching][git-flow]\n",
            link: "[git-flow]: <https://github.com/sean-hut/contributing-rules/blob/develop/rules/preparation-for-commits/git-flow-branching.md>\n",
        verbose: "[Info] Included git-flow rule",
        },
        Rule {
            flag: arguments.occurrences_of("fetch-merge") > 0,
            rule: "- [Fetch and Merge Before Committing][fetch-merge]\n",
            link: "[fetch-merge]: <https://github.com/sean-hut/contributing-rules/blob/develop/rules/preparation-for-commits/fetch-and-merge-before-committing.md>\n",
            verbose: "[Info] Included fetch and merge rule",
        },
        Rule {
            flag: arguments.occurrences_of("documentation") > 0,
            rule: "- [Update Documentation][docs]\n",
            link: "[docs]: <https://github.com/sean-hut/contributing-rules/blob/develop/rules/preparation-for-commits/update-documentation.md>\n",
            verbose: "[Info] Included documentation rule",
        },
        Rule {
            flag: arguments.occurrences_of("test") > 0,
            rule: "- [Update Tests][tests]\n",
            link: "[tests]: <https://github.com/sean-hut/contributing-rules/blob/develop/rules/preparation-for-commits/update-tests.md>\n",
            verbose: "[Info] Included test rule",
        },
        Rule {
            flag: arguments.occurrences_of("make") > 0,
            rule: "- [Run Make][run-make]\n",
            link: "[run-make]: <https://github.com/sean-hut/contributing-rules/blob/develop/rules/preparation-for-commits/run-make.md>\n",
            verbose: "[Info] Included make rule",
        },
    ]
}

pub fn any_preparation(arguments: &ArgMatches) -> bool {
    preparation(&arguments).iter().map(|x| x.flag).any(|x| x)
}
