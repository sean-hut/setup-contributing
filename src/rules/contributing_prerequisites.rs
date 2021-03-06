use clap::ArgMatches;

use crate::rules::rule::Rule;

pub const PREREQUISITE_HEADING: &str = "## Contributing Prerequisites\n\n";

pub fn prerequisites<'a>(arguments: &'a ArgMatches) -> Vec<Rule<'a>> {
    vec![
        Rule {
            flag: arguments.is_present("elliptic-curve"),
            rule: "- [GPG2 Elliptic-Curve Signing Key][elliptic-curve]\n",
        link: "[elliptic-curve]: <https://github.com/sean-hut/contributing-rules/blob/develop/rules/contributing-prerequisites/gpg2-eliptic-curve-signing-key.md>\n",
        verbose: "[Info] Included eliptic-curve signing key rule",
        },
        Rule {
            flag: arguments.is_present("public-key"),
            rule: "- [Provide GPG2 Public Key][public-key]\n",
            link: "[public-key]: <https://github.com/sean-hut/contributing-rules/blob/develop/rules/contributing-prerequisites/gpg2-public-key.md>\n",
            verbose: "[Info] Included provide public key rule",
        },
        Rule {
            flag: arguments.is_present("contributor-agreement"),
            rule: "- [Contributor Agreement][agreement]\n",
            link: "[agreement]: <https://github.com/sean-hut/contributing-rules/blob/develop/rules/contributing-prerequisites/contributor-agreement.md>\n",
            verbose: "[Info] Included contributor agreement rule",
        },
        Rule {
            flag: arguments.is_present("git-config-standard"),
            rule: "- [Git Configuration][git-config]\n",
            link: "[git-config]: <https://github.com/sean-hut/contributing-rules/blob/develop/rules/contributing-prerequisites/git-configuration/git-configuration.md>\n",
            verbose: "[Info] Included standard Git configuration rule",
        },
        Rule {
            flag: arguments.is_present("git-config-gpg-signoff"),
            rule: "- [Git Configuration][git-config-gpg-signoff]\n",
            link: "[git-config-gpg-signoff]: <https://github.com/sean-hut/contributing-rules/blob/develop/rules/contributing-prerequisites/git-configuration/git-configuration-gpg-signoff.md>\n",
            verbose: "[Info] Included Git configuration with gpg signing and signoff rule",
        },
        Rule {
            flag: arguments.is_present("pre-commit-hook"),
            rule: "- [Git `pre-commit` Hook][pre-commit]\n",
            link: "[pre-commit]: <https://github.com/sean-hut/contributing-rules/blob/develop/rules/contributing-prerequisites/git-hook/pre-commit.md>\n",
            verbose: "[Info] Included Git pre-commit hook rule",
        },
        Rule {
            flag: arguments.is_present("commit-message-hook"),
            rule: "- [Amaranth Git `commit-msg` Hook][commit-msg]\n",
            link: "[commit-msg]: <https://github.com/sean-hut/contributing-rules/blob/develop/rules/contributing-prerequisites/git-hook/commit-msg.md>\n",
            verbose: "[Info] Included Amaranth Git commit-msg hook rule",
        },
    ]
}

pub fn any_prerequisites(arguments: &ArgMatches) -> bool {
    prerequisites(&arguments).iter().map(|x| x.flag).any(|x| x)
}
