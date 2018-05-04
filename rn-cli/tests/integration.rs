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
}
