extern crate assert_cli;

#[cfg(test)]
mod integration {
    use assert_cli::Assert;

    #[test]
    fn it_can_generate_a_random_name() {
        Assert::main_binary().succeeds().unwrap();
    }

    #[test]
    fn it_can_generate_a_name_based_on_a_sha() {
        Assert::main_binary()
            .with_args(&["017020733fecef58761259d5d307c83876f9b428"])
            .succeeds()
            .stdout()
            .contains("issuably twinning verso")
            .unwrap();
    }

    #[test]
    fn it_can_generate_a_name_based_on_a_sha_with_casing() {
        Assert::main_binary()
            .with_args(&[
                "--format",
                "camel",
                "017020733fecef58761259d5d307c83876f9b428",
            ])
            .succeeds()
            .stdout()
            .contains("issuablyTwinningVerso")
            .unwrap();
        Assert::main_binary()
            .with_args(&["-f", "camel", "017020733fecef58761259d5d307c83876f9b428"])
            .succeeds()
            .stdout()
            .contains("issuablyTwinningVerso")
            .unwrap();
    }

    #[test]
    fn it_can_generate_a_name_based_on_a_sha_from_stdin() {
        Assert::main_binary()
            .stdin("017020733fecef58761259d5d307c83876f9b428")
            .succeeds()
            .stdout()
            .contains("issuably twinning verso")
            .unwrap();
    }

    #[test]
    fn it_can_generate_a_name_based_on_a_sha_from_stdin_with_casing() {
        Assert::main_binary()
            .with_args(&["--format", "camel"])
            .stdin("017020733fecef58761259d5d307c83876f9b428")
            .succeeds()
            .stdout()
            .contains("issuablyTwinningVerso")
            .unwrap();
        Assert::main_binary()
            .with_args(&["-f", "camel"])
            .stdin("017020733fecef58761259d5d307c83876f9b428")
            .succeeds()
            .stdout()
            .contains("issuablyTwinningVerso")
            .unwrap();
    }

    #[test]
    fn formats_sets_of_words() {
        Assert::main_binary()
            .with_args(&["list", "-i", "nouns", "-f", "csv"])
            .succeeds()
            .stdout()
            .contains("noun,aba,3230")
            .unwrap();
        Assert::main_binary()
            .with_args(&["list", "-i", "nouns", "--format", "csv"])
            .succeeds()
            .stdout()
            .contains("noun,aba,3230")
            .unwrap();
        Assert::main_binary()
            .with_args(&["list", "-i", "nouns", "-f", "fixed"])
            .succeeds()
            .stdout()
            .contains("noun aba                  3230")
            .unwrap();
        Assert::main_binary()
            .with_args(&["list", "-i", "nouns", "--format", "fixed"])
            .succeeds()
            .stdout()
            .contains("noun aba                  3230")
            .unwrap();
    }

    #[test]
    fn lists_sets_of_words() {
        macro_rules! test {
            (list $($type:expr),*; contains $($word:expr),*) => {
                Assert::main_binary()
                    .with_args(&["list" $(, "--include", $type)*])
                    .succeeds()
                    $(
                    .stdout()
                    .contains($word)
                    )*
                    .unwrap();
                Assert::main_binary()
                    .with_args(&["list", "-i" $(, $type)*])
                    .succeeds()
                    $(
                    .stdout()
                    .contains($word)
                    )*
                    .unwrap();
            };

            (list contains $($word:expr),*) => {
                Assert::main_binary()
                    .with_args(&["list"])
                    .succeeds()
                    $(
                    .stdout()
                    .contains($word)
                    )*
                    .unwrap();
            };
        }

        test!(list contains "verso", "twinning", "issuably");
        test!(list "nouns"; contains "verso");
        test!(list "n"; contains "verso");
        test!(list "adjectives"; contains "twinning");
        test!(list "adj"; contains "twinning");
        test!(list "adverbs"; contains "issuably");
        test!(list "adv"; contains "issuably");
        test!(list "nouns", "adv"; contains "issuably", "verso");
        test!(list "nouns", "adj"; contains "twinning", "verso");
        test!(list "adv", "adj"; contains "twinning", "issuably");
    }
}
