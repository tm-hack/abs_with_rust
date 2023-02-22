mod tests {
    use cli_test_dir::*;
    const BIN: &'static str = "./main";
    #[test]
    fn case1() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r#"4
10
8
8
6
"#,
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            "3
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
                r#"3
15
15
15
"#,
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            "1
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
                r#"7
50
30
50
100
50
80
30
"#,
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            "4
"
        );
        assert!(output.stderr_str().is_empty());
    }
}
