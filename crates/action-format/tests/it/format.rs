use crate::action_format_snapshot;
use crate::common::TestContext;

#[test]
fn test_format_basic_workflow() {
    let context = TestContext::new();
    context.workflow(
        "ci.yml",
        r"name: CI
on: push
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Build
        run: cargo build
",
    );

    action_format_snapshot!(context.filters(), context.command(), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Reformatted: ci.yml

    ----- stderr -----
    ");

    let content = context.read_workflow("ci.yml");
    insta::assert_snapshot!(content, @r"
    name: CI
    on: push
    jobs:
      build:
        runs-on: ubuntu-latest
        steps:
          - uses: actions/checkout@v4

          - name: Build
            run: cargo build
    ");
}

#[test]
fn test_format_step_separation() {
    let context = TestContext::new();
    context.workflow(
        "ci.yml",
        r"name: CI
on: push
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Build
        run: cargo build
      - name: Test
        run: cargo test
",
    );

    context.command().assert().success();

    let content = context.read_workflow("ci.yml");
    insta::assert_snapshot!(content, @r"
    name: CI
    on: push
    jobs:
      build:
        runs-on: ubuntu-latest
        steps:
          - uses: actions/checkout@v4

          - name: Build
            run: cargo build

          - name: Test
            run: cargo test
    ");
}

#[test]
fn test_format_preserves_comments() {
    let context = TestContext::new();
    context.workflow(
        "ci.yml",
        r"# This is a workflow
name: CI
on: push
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      # Checkout the code
      - uses: actions/checkout@v4
      # Build step
      - name: Build
        run: cargo build
",
    );

    context.command().assert().success();

    let content = context.read_workflow("ci.yml");
    insta::assert_snapshot!(content, @r"
    # This is a workflow
    name: CI
    on: push
    jobs:
      build:
        runs-on: ubuntu-latest
        steps:
          # Checkout the code
          - uses: actions/checkout@v4
          # Build step

          - name: Build
            run: cargo build
    ");
}

#[test]
fn test_format_normalizes_4_space_indent() {
    let context = TestContext::new();
    context.workflow(
        "ci.yml",
        "name: CI\non: push\njobs:\n    build:\n        runs-on: ubuntu-latest\n",
    );

    context.command().assert().success();

    let content = context.read_workflow("ci.yml");
    insta::assert_snapshot!(content, @r"
    name: CI
    on: push
    jobs:
      build:
        runs-on: ubuntu-latest
    ");
}

#[test]
fn test_check_mode_no_changes() {
    let context = TestContext::new();
    context.workflow(
        "ci.yml",
        r"name: CI
on: push
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Build
        run: cargo build
",
    );

    action_format_snapshot!(context.filters(), context.command().arg("--check"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");
}

#[test]
fn test_check_mode_with_changes() {
    let context = TestContext::new();
    context.workflow(
        "ci.yml",
        r"name: CI
on: push
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Build
        run: cargo build
",
    );

    action_format_snapshot!(context.filters(), context.command().arg("--check"), @r"
    success: false
    exit_code: 1
    ----- stdout -----
    Would reformat: ci.yml

    ----- stderr -----
    ");
}
