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
        macro_rules! test_list_command {
            () => {};

            (when including $($type:tt),* contains $($word:expr),*;$($tail:tt)*) => {
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
                test_list_command!($($tail)*);
            };

            (without args contains $($word:expr),*;$($tail:tt)*) => {
                Assert::main_binary()
                    .with_args(&["list"])
                    .succeeds()
                    $(
                    .stdout()
                    .contains($word)
                    )*
                    .unwrap();
                test_list_command!($($tail)*);
            };
        }

        test_list_command! {
            without args contains "verso", "twinning", "issuably";

            when including "nouns"                  contains "verso";
            when including "n"                      contains "verso";
            when including "adjectives"             contains "twinning";
            when including "adj"                    contains "twinning";
            when including "adverbs"                contains "issuably";
            when including "adv"                    contains "issuably";
            when including "nouns", "adv"           contains "issuably", "verso";
            when including "nouns", "adj"           contains "twinning", "verso";
            when including "adv", "adj"             contains "twinning", "issuably";
            when including "nouns", "adv", "adj"    contains "verso", "twinning", "issuably";
        };
    }
}
