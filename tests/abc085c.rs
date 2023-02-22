mod tests {
    use cli_test_dir::*;
    const BIN: &'static str = "./main";
    #[test]
    fn case1() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r#"9 45000
"#,
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            "4 0 5
"
        );
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    fn case2() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r#"20 196000
"#,
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            "-1 -1 -1
"
        );
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    fn case3() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r#"1000 1234000
"#,
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            "14 27 959
"
        );
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    fn case4() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r#"2000 20000000
"#,
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            "2000 0 0
"
        );
        assert!(output.stderr_str().is_empty());
    }
}
