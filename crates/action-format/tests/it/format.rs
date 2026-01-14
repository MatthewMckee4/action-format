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
    Reformatted: .github/workflows/ci.yml

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
fn test_format_job_separation() {
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
  test:
    runs-on: ubuntu-latest
    needs: build
    steps:
      - uses: actions/checkout@v4
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

      test:
        runs-on: ubuntu-latest
        needs: build
        steps:
          - uses: actions/checkout@v4
    ");
}

#[test]
fn test_format_job_separation_with_comments() {
    let context = TestContext::new();
    context.workflow(
        "ci.yml",
        r"name: CI
on: push
jobs:
  # Build job
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
  # Test job
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
",
    );

    context.command().assert().success();

    let content = context.read_workflow("ci.yml");
    insta::assert_snapshot!(content, @r"
    name: CI
    on: push
    jobs:
      # Build job
      build:
        runs-on: ubuntu-latest
        steps:
          - uses: actions/checkout@v4
      # Test job

      test:
        runs-on: ubuntu-latest
        steps:
          - uses: actions/checkout@v4
    ");
}

#[test]
fn test_format_job_separation_preserves_existing_blank() {
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

  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
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

      test:
        runs-on: ubuntu-latest
        steps:
          - uses: actions/checkout@v4
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
    Would reformat: .github/workflows/ci.yml

    ----- stderr -----
    ");
}

#[test]
fn test_format_multi_job_workflow() {
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
  test:
    runs-on: ubuntu-latest
    needs: build
    steps:
      - uses: actions/checkout@v4
      - name: Test
        run: cargo test
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Lint
        run: cargo clippy
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

      test:
        runs-on: ubuntu-latest
        needs: build
        steps:
          - uses: actions/checkout@v4

          - name: Test
            run: cargo test

      lint:
        runs-on: ubuntu-latest
        steps:
          - uses: actions/checkout@v4

          - name: Lint
            run: cargo clippy
    ");
}

#[test]
fn test_format_matrix_strategy() {
    let context = TestContext::new();
    context.workflow(
        "ci.yml",
        r"name: CI
on: push
jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        rust: [stable, beta, nightly]
      fail-fast: false
    steps:
      - uses: actions/checkout@v4
      - name: Setup Rust
        uses: dtolnay/rust-toolchain@master
        with:
          toolchain: ${{ matrix.rust }}
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
      test:
        runs-on: ${{ matrix.os }}
        strategy:
          matrix:
            os: [ubuntu-latest, macos-latest, windows-latest]
            rust: [stable, beta, nightly]
          fail-fast: false
        steps:
          - uses: actions/checkout@v4

          - name: Setup Rust
            uses: dtolnay/rust-toolchain@master
            with:
              toolchain: ${{ matrix.rust }}

          - name: Test
            run: cargo test
    ");
}

#[test]
fn test_format_environment_variables() {
    let context = TestContext::new();
    context.workflow(
        "ci.yml",
        r"name: CI
on: push
env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1
jobs:
  build:
    runs-on: ubuntu-latest
    env:
      BUILD_MODE: release
    steps:
      - uses: actions/checkout@v4
      - name: Build
        env:
          RUSTFLAGS: -D warnings
        run: cargo build --release
",
    );

    context.command().assert().success();

    let content = context.read_workflow("ci.yml");
    insta::assert_snapshot!(content, @r"
    name: CI
    on: push
    env:
      CARGO_TERM_COLOR: always
      RUST_BACKTRACE: 1
    jobs:
      build:
        runs-on: ubuntu-latest
        env:
          BUILD_MODE: release
        steps:
          - uses: actions/checkout@v4

          - name: Build
            env:
              RUSTFLAGS: -D warnings
            run: cargo build --release
    ");
}

#[test]
fn test_format_secrets_usage() {
    let context = TestContext::new();
    context.workflow(
        "deploy.yml",
        r"name: Deploy
on:
  push:
    branches: [main]
jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Deploy
        env:
          API_KEY: ${{ secrets.API_KEY }}
          DEPLOY_TOKEN: ${{ secrets.DEPLOY_TOKEN }}
        run: ./deploy.sh
",
    );

    context.command().assert().success();

    let content = context.read_workflow("deploy.yml");
    insta::assert_snapshot!(content, @r"
    name: Deploy
    on:
      push:
        branches: [main]
    jobs:
      deploy:
        runs-on: ubuntu-latest
        steps:
          - uses: actions/checkout@v4

          - name: Deploy
            env:
              API_KEY: ${{ secrets.API_KEY }}
              DEPLOY_TOKEN: ${{ secrets.DEPLOY_TOKEN }}
            run: ./deploy.sh
    ");
}

#[test]
fn test_format_conditional_steps() {
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
      - name: Upload coverage
        if: github.event_name == 'push' && github.ref == 'refs/heads/main'
        run: ./upload-coverage.sh
      - name: Notify on failure
        if: failure()
        run: echo 'Build failed!'
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

          - name: Upload coverage
            if: github.event_name == 'push' && github.ref == 'refs/heads/main'
            run: ./upload-coverage.sh

          - name: Notify on failure
            if: failure()
            run: echo 'Build failed!'
    ");
}

#[test]
fn test_format_multiple_triggers() {
    let context = TestContext::new();
    context.workflow(
        "ci.yml",
        r"name: CI
on:
  push:
    branches: [main, develop]
    paths:
      - 'src/**'
      - 'Cargo.toml'
  pull_request:
    branches: [main]
  workflow_dispatch:
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Build
        run: cargo build
",
    );

    context.command().assert().success();

    let content = context.read_workflow("ci.yml");
    insta::assert_snapshot!(content, @r"
    name: CI
    on:
      push:
        branches: [main, develop]
        paths:
          - 'src/**'
          - 'Cargo.toml'
      pull_request:
        branches: [main]
      workflow_dispatch:
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
fn test_format_services() {
    let context = TestContext::new();
    context.workflow(
        "ci.yml",
        r"name: CI
on: push
jobs:
  test:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:15
        env:
          POSTGRES_PASSWORD: postgres
        ports:
          - 5432:5432
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
      redis:
        image: redis:7
        ports:
          - 6379:6379
    steps:
      - uses: actions/checkout@v4
      - name: Run tests
        run: cargo test
",
    );

    context.command().assert().success();

    let content = context.read_workflow("ci.yml");
    insta::assert_snapshot!(content, @r"
    name: CI
    on: push
    jobs:
      test:
        runs-on: ubuntu-latest
        services:
          postgres:
            image: postgres:15
            env:
              POSTGRES_PASSWORD: postgres
            ports:
              - 5432:5432
            options: >-
              --health-cmd pg_isready
              --health-interval 10s
              --health-timeout 5s
              --health-retries 5
          redis:
            image: redis:7
            ports:
              - 6379:6379
        steps:
          - uses: actions/checkout@v4

          - name: Run tests
            run: cargo test
    ");
}

#[test]
fn test_format_container_job() {
    let context = TestContext::new();
    context.workflow(
        "ci.yml",
        r"name: CI
on: push
jobs:
  build:
    runs-on: ubuntu-latest
    container:
      image: rust:1.75
      env:
        CARGO_HOME: /cargo
      volumes:
        - /cargo:/cargo
      options: --user root
    steps:
      - uses: actions/checkout@v4
      - name: Build
        run: cargo build
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
        container:
          image: rust:1.75
          env:
            CARGO_HOME: /cargo
          volumes:
            - /cargo:/cargo
          options: --user root
        steps:
          - uses: actions/checkout@v4

          - name: Build
            run: cargo build
    ");
}

#[test]
fn test_format_multiline_run() {
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
      - name: Setup
        run: |
          echo 'Setting up...'
          mkdir -p build
          cp config.example.yml config.yml
      - name: Build
        run: |
          cd build
          cmake ..
          make -j$(nproc)
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

          - name: Setup
            run: |
              echo 'Setting up...'
              mkdir -p build
              cp config.example.yml config.yml

          - name: Build
            run: |
              cd build
              cmake ..
              make -j$(nproc)
    ");
}

#[test]
fn test_format_outputs_between_steps() {
    let context = TestContext::new();
    context.workflow(
        "ci.yml",
        r"name: CI
on: push
jobs:
  version:
    runs-on: ubuntu-latest
    outputs:
      version: ${{ steps.get_version.outputs.version }}
    steps:
      - uses: actions/checkout@v4
      - name: Get version
        id: get_version
        run: echo 'version=1.0.0' >> $GITHUB_OUTPUT
  build:
    runs-on: ubuntu-latest
    needs: version
    steps:
      - uses: actions/checkout@v4
      - name: Build
        run: echo 'Building version ${{ needs.version.outputs.version }}'
",
    );

    context.command().assert().success();

    let content = context.read_workflow("ci.yml");
    insta::assert_snapshot!(content, @r"
    name: CI
    on: push
    jobs:
      version:
        runs-on: ubuntu-latest
        outputs:
          version: ${{ steps.get_version.outputs.version }}
        steps:
          - uses: actions/checkout@v4

          - name: Get version
            id: get_version
            run: echo 'version=1.0.0' >> $GITHUB_OUTPUT

      build:
        runs-on: ubuntu-latest
        needs: version
        steps:
          - uses: actions/checkout@v4

          - name: Build
            run: echo 'Building version ${{ needs.version.outputs.version }}'
    ");
}

#[test]
fn test_format_concurrency() {
    let context = TestContext::new();
    context.workflow(
        "ci.yml",
        r"name: CI
on: push
concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Build
        run: cargo build
",
    );

    context.command().assert().success();

    let content = context.read_workflow("ci.yml");
    insta::assert_snapshot!(content, @r"
    name: CI
    on: push
    concurrency:
      group: ${{ github.workflow }}-${{ github.ref }}
      cancel-in-progress: true
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
fn test_format_permissions() {
    let context = TestContext::new();
    context.workflow(
        "ci.yml",
        r"name: CI
on: push
permissions:
  contents: read
  packages: write
  id-token: write
jobs:
  build:
    runs-on: ubuntu-latest
    permissions:
      contents: read
    steps:
      - uses: actions/checkout@v4
      - name: Build
        run: cargo build
",
    );

    context.command().assert().success();

    let content = context.read_workflow("ci.yml");
    insta::assert_snapshot!(content, @r"
    name: CI
    on: push
    permissions:
      contents: read
      packages: write
      id-token: write
    jobs:
      build:
        runs-on: ubuntu-latest
        permissions:
          contents: read
        steps:
          - uses: actions/checkout@v4

          - name: Build
            run: cargo build
    ");
}

#[test]
fn test_format_timeout_and_continue_on_error() {
    let context = TestContext::new();
    context.workflow(
        "ci.yml",
        r"name: CI
on: push
jobs:
  build:
    runs-on: ubuntu-latest
    timeout-minutes: 30
    steps:
      - uses: actions/checkout@v4
      - name: Flaky test
        continue-on-error: true
        timeout-minutes: 5
        run: ./flaky-test.sh
      - name: Build
        run: cargo build
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
        timeout-minutes: 30
        steps:
          - uses: actions/checkout@v4

          - name: Flaky test
            continue-on-error: true
            timeout-minutes: 5
            run: ./flaky-test.sh

          - name: Build
            run: cargo build
    ");
}

#[test]
fn test_format_working_directory_and_shell() {
    let context = TestContext::new();
    context.workflow(
        "ci.yml",
        r"name: CI
on: push
defaults:
  run:
    working-directory: ./src
    shell: bash
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Build in subdir
        working-directory: ./packages/core
        run: npm run build
      - name: PowerShell step
        shell: pwsh
        run: Write-Host 'Hello from PowerShell'
",
    );

    context.command().assert().success();

    let content = context.read_workflow("ci.yml");
    insta::assert_snapshot!(content, @r"
    name: CI
    on: push
    defaults:
      run:
        working-directory: ./src
        shell: bash
    jobs:
      build:
        runs-on: ubuntu-latest
        steps:
          - uses: actions/checkout@v4

          - name: Build in subdir
            working-directory: ./packages/core
            run: npm run build

          - name: PowerShell step
            shell: pwsh
            run: Write-Host 'Hello from PowerShell'
    ");
}

#[test]
fn test_format_artifacts() {
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
        run: cargo build --release
      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: binary
          path: target/release/myapp
          retention-days: 5
  test:
    runs-on: ubuntu-latest
    needs: build
    steps:
      - name: Download artifact
        uses: actions/download-artifact@v4
        with:
          name: binary
      - name: Test binary
        run: ./myapp --version
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
            run: cargo build --release

          - name: Upload artifact
            uses: actions/upload-artifact@v4
            with:
              name: binary
              path: target/release/myapp
              retention-days: 5

      test:
        runs-on: ubuntu-latest
        needs: build
        steps:
          - name: Download artifact
            uses: actions/download-artifact@v4
            with:
              name: binary

          - name: Test binary
            run: ./myapp --version
    ");
}

#[test]
fn test_format_caching() {
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
      - name: Cache cargo registry
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: |
            ${{ runner.os }}-cargo-
      - name: Build
        run: cargo build
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

          - name: Cache cargo registry
            uses: actions/cache@v4
            with:
              path: |
                ~/.cargo/registry
                ~/.cargo/git
                target
              key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
              restore-keys: |
                ${{ runner.os }}-cargo-

          - name: Build
            run: cargo build
    ");
}

#[test]
fn test_format_workflow_dispatch_inputs() {
    let context = TestContext::new();
    context.workflow(
        "deploy.yml",
        r"name: Deploy
on:
  workflow_dispatch:
    inputs:
      environment:
        description: 'Environment to deploy to'
        required: true
        default: 'staging'
        type: choice
        options:
          - staging
          - production
      dry_run:
        description: 'Perform a dry run'
        required: false
        type: boolean
        default: false
jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Deploy
        run: ./deploy.sh ${{ inputs.environment }}
        env:
          DRY_RUN: ${{ inputs.dry_run }}
",
    );

    context.command().assert().success();

    let content = context.read_workflow("deploy.yml");
    insta::assert_snapshot!(content, @r"
    name: Deploy
    on:
      workflow_dispatch:
        inputs:
          environment:
            description: 'Environment to deploy to'
            required: true
            default: 'staging'
            type: choice
            options:
              - staging
              - production
          dry_run:
            description: 'Perform a dry run'
            required: false
            type: boolean
            default: false
    jobs:
      deploy:
        runs-on: ubuntu-latest
        steps:
          - uses: actions/checkout@v4

          - name: Deploy
            run: ./deploy.sh ${{ inputs.environment }}
            env:
              DRY_RUN: ${{ inputs.dry_run }}
    ");
}

#[test]
fn test_format_schedule_trigger() {
    let context = TestContext::new();
    context.workflow(
        "nightly.yml",
        r"name: Nightly Build
on:
  schedule:
    - cron: '0 2 * * *'
    - cron: '0 14 * * *'
  workflow_dispatch:
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Build
        run: cargo build --release
",
    );

    context.command().assert().success();

    let content = context.read_workflow("nightly.yml");
    insta::assert_snapshot!(content, @r"
    name: Nightly Build
    on:
      schedule:
        - cron: '0 2 * * *'
        - cron: '0 14 * * *'
      workflow_dispatch:
    jobs:
      build:
        runs-on: ubuntu-latest
        steps:
          - uses: actions/checkout@v4

          - name: Build
            run: cargo build --release
    ");
}

#[test]
fn test_format_reusable_workflow_caller() {
    let context = TestContext::new();
    context.workflow(
        "ci.yml",
        r"name: CI
on: push
jobs:
  call-build:
    uses: ./.github/workflows/build.yml
    with:
      target: release
    secrets:
      deploy_key: ${{ secrets.DEPLOY_KEY }}
  call-external:
    uses: org/repo/.github/workflows/shared.yml@main
    with:
      config: production
",
    );

    context.command().assert().success();

    let content = context.read_workflow("ci.yml");
    insta::assert_snapshot!(content, @r"
    name: CI
    on: push
    jobs:
      call-build:
        uses: ./.github/workflows/build.yml
        with:
          target: release
        secrets:
          deploy_key: ${{ secrets.DEPLOY_KEY }}

      call-external:
        uses: org/repo/.github/workflows/shared.yml@main
        with:
          config: production
    ");
}

#[test]
fn test_format_reusable_workflow_definition() {
    let context = TestContext::new();
    context.workflow(
        "build.yml",
        r"name: Reusable Build
on:
  workflow_call:
    inputs:
      target:
        required: true
        type: string
    secrets:
      deploy_key:
        required: true
    outputs:
      artifact_url:
        description: 'URL of the built artifact'
        value: ${{ jobs.build.outputs.url }}
jobs:
  build:
    runs-on: ubuntu-latest
    outputs:
      url: ${{ steps.upload.outputs.url }}
    steps:
      - uses: actions/checkout@v4
      - name: Build
        run: cargo build --${{ inputs.target }}
      - name: Upload
        id: upload
        run: echo 'url=https://example.com/artifact' >> $GITHUB_OUTPUT
",
    );

    context.command().assert().success();

    let content = context.read_workflow("build.yml");
    insta::assert_snapshot!(content, @r"
    name: Reusable Build
    on:
      workflow_call:
        inputs:
          target:
            required: true
            type: string
        secrets:
          deploy_key:
            required: true
        outputs:
          artifact_url:
            description: 'URL of the built artifact'
            value: ${{ jobs.build.outputs.url }}
    jobs:
      build:
        runs-on: ubuntu-latest
        outputs:
          url: ${{ steps.upload.outputs.url }}
        steps:
          - uses: actions/checkout@v4

          - name: Build
            run: cargo build --${{ inputs.target }}

          - name: Upload
            id: upload
            run: echo 'url=https://example.com/artifact' >> $GITHUB_OUTPUT
    ");
}

#[test]
fn test_format_yaml_anchors() {
    let context = TestContext::new();
    context.workflow(
        "ci.yml",
        r"name: CI
on: push
env:
  COMMON_VAR: &common_value 'shared'
jobs:
  build:
    runs-on: ubuntu-latest
    env:
      MY_VAR: *common_value
    steps:
      - uses: actions/checkout@v4
      - name: Build
        run: echo $MY_VAR
",
    );

    context.command().assert().success();

    let content = context.read_workflow("ci.yml");
    insta::assert_snapshot!(content, @r"
    name: CI
    on: push
    env:
      COMMON_VAR: &common_value 'shared'
    jobs:
      build:
        runs-on: ubuntu-latest
        env:
          MY_VAR: *common_value
        steps:
          - uses: actions/checkout@v4

          - name: Build
            run: echo $MY_VAR
    ");
}

#[test]
fn test_format_complex_with_block() {
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
        with:
          fetch-depth: 0
          submodules: recursive
          token: ${{ secrets.PAT }}
      - uses: docker/build-push-action@v5
        with:
          context: .
          push: true
          tags: |
            myapp:latest
            myapp:${{ github.sha }}
          build-args: |
            VERSION=${{ github.ref_name }}
            COMMIT=${{ github.sha }}
          cache-from: type=gha
          cache-to: type=gha,mode=max
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
            with:
              fetch-depth: 0
              submodules: recursive
              token: ${{ secrets.PAT }}

          - uses: docker/build-push-action@v5
            with:
              context: .
              push: true
              tags: |
                myapp:latest
                myapp:${{ github.sha }}
              build-args: |
                VERSION=${{ github.ref_name }}
                COMMIT=${{ github.sha }}
              cache-from: type=gha
              cache-to: type=gha,mode=max
    ");
}

#[test]
fn test_format_multiple_workflow_files() {
    let context = TestContext::new();
    context.workflow(
        "a_ci.yml",
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
    context.workflow(
        "b_release.yml",
        r"name: Release
on:
  push:
    tags: ['v*']
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Release
        run: cargo publish
",
    );

    action_format_snapshot!(context.filters(), context.command(), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Reformatted: .github/workflows/a_ci.yml
    Reformatted: .github/workflows/b_release.yml

    ----- stderr -----
    ");
}

#[test]
fn test_format_empty_steps() {
    let context = TestContext::new();
    context.workflow(
        "ci.yml",
        r"name: CI
on: push
jobs:
  build:
    runs-on: ubuntu-latest
    steps: []
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
        steps: []
    ");
}

#[test]
fn test_format_deeply_nested_structure() {
    let context = TestContext::new();
    context.workflow(
        "ci.yml",
        r"name: CI
on:
  push:
    branches:
      - main
      - 'release/**'
    paths-ignore:
      - '**.md'
      - 'docs/**'
jobs:
  build:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            features: default
          - os: macos-latest
            target: aarch64-apple-darwin
            features: vendored
    steps:
      - uses: actions/checkout@v4
      - name: Build
        run: cargo build --target ${{ matrix.target }} --features ${{ matrix.features }}
",
    );

    context.command().assert().success();

    let content = context.read_workflow("ci.yml");
    insta::assert_snapshot!(content, @r"
    name: CI
    on:
      push:
        branches:
          - main
          - 'release/**'
        paths-ignore:
          - '**.md'
          - 'docs/**'
    jobs:
      build:
        runs-on: ubuntu-latest
        strategy:
          matrix:
            include:
              - os: ubuntu-latest
                target: x86_64-unknown-linux-gnu
                features: default
              - os: macos-latest
                target: aarch64-apple-darwin
                features: vendored
        steps:
          - uses: actions/checkout@v4

          - name: Build
            run: cargo build --target ${{ matrix.target }} --features ${{ matrix.features }}
    ");
}

#[test]
fn test_format_docker_action() {
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
      - uses: docker://alpine:3.19
        with:
          args: echo 'Hello from Alpine'
      - uses: docker://ghcr.io/org/action:v1
        env:
          INPUT_TOKEN: ${{ secrets.TOKEN }}
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

          - uses: docker://alpine:3.19
            with:
              args: echo 'Hello from Alpine'

          - uses: docker://ghcr.io/org/action:v1
            env:
              INPUT_TOKEN: ${{ secrets.TOKEN }}
    ");
}

#[test]
fn test_format_local_action() {
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
      - uses: ./.github/actions/setup
        with:
          version: '1.0'
      - uses: ./custom-action
        id: custom
      - name: Use output
        run: echo ${{ steps.custom.outputs.result }}
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

          - uses: ./.github/actions/setup
            with:
              version: '1.0'

          - uses: ./custom-action
            id: custom

          - name: Use output
            run: echo ${{ steps.custom.outputs.result }}
    ");
}

#[test]
fn test_format_inline_comment_preservation() {
    let context = TestContext::new();
    context.workflow(
        "ci.yml",
        r"name: CI  # Main workflow
on: push  # Trigger on push
jobs:
  build:  # Build job
    runs-on: ubuntu-latest  # Use latest Ubuntu
    steps:
      - uses: actions/checkout@v4  # Checkout code
      - name: Build  # Build step
        run: cargo build  # Run cargo
",
    );

    context.command().assert().success();

    let content = context.read_workflow("ci.yml");
    insta::assert_snapshot!(content, @r"
    name: CI  # Main workflow
    on: push  # Trigger on push
    jobs:
      build:  # Build job
        runs-on: ubuntu-latest  # Use latest Ubuntu
        steps:
          - uses: actions/checkout@v4  # Checkout code

          - name: Build  # Build step
            run: cargo build  # Run cargo
    ");
}

#[test]
fn test_diff_mode() {
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

    action_format_snapshot!(context.filters(), context.command().arg("--diff"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Source: .github/workflows/ci.yml
    ────────────┬───────────────────────────────────────────────────────────────────
        4     4 │   build:
        5     5 │     runs-on: ubuntu-latest
        6     6 │     steps:
        7     7 │       - uses: actions/checkout@v4
              8 │+
        8     9 │       - name: Build
        9    10 │         run: cargo build
    ────────────┴───────────────────────────────────────────────────────────────────

    ----- stderr -----
    ");

    // Original file left unchanged
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
