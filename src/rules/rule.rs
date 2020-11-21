pub struct Rule<'a> {
    pub flag: bool,
    pub rule: &'a str,
    pub link: &'a str,
    pub verbose: &'a str,
}
