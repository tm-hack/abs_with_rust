mod tests {
    use cli_test_dir::*;
    const BIN: &'static str = "./main";
    #[test]
    fn case1() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r#"2
2
2
100
"#,
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            "2
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
                r#"5
1
0
150
"#,
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            "0
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
                r#"30
40
50
6000
"#,
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            "213
"
        );
        assert!(output.stderr_str().is_empty());
    }
}
