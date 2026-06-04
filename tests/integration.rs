use std::process::Command;

fn runlikers(args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_runlikers"))
        .args(args)
        .output()
        .expect("failed to run runlikers");
    assert!(output.status.success(), "runlikers failed: {}", String::from_utf8_lossy(&output.stderr));
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn expect_contains(hay: &str, needle: &str, fixture: &str) {
    assert!(hay.contains(needle), "fixture {}: expected '{}' not found in:\n{}", fixture, needle, hay);
}

fn expect_not_contains(hay: &str, needle: &str, fixture: &str) {
    assert!(!hay.contains(needle), "fixture {}: unexpected '{}' found in:\n{}", fixture, needle, hay);
}

fn starts_with(hay: &str, prefix: &str, fixture: &str) {
    assert!(hay.starts_with(prefix), "fixture {}: expected to start with '{}', got:\n{}", fixture, prefix, hay);
}

#[test]
fn fixture1() {
    let out = runlikers(&["--pretty", "runlike_fixture1"]);
    starts_with(&out, "docker run", "1");
    expect_contains(&out, "-p 300 \\", "1");
    expect_contains(&out, "-p 400:400 \\", "1");
    expect_contains(&out, "--expose=1000 \\", "1");
    expect_contains(&out, "-p 301/udp \\", "1");
    expect_contains(&out, "--dns=8.8.8.8 \\", "1");
    expect_contains(&out, "--dns=8.8.4.4 \\", "1");
    expect_contains(&out, "-p 503:502/udp \\", "1");
    expect_contains(&out, "-p 127.0.0.1:601:600/udp \\", "1");
    expect_contains(&out, "-t \\", "1");
    expect_contains(&out, "--hostname=Essos \\", "1");
    expect_contains(&out, "--privileged \\", "1");
    expect_contains(&out, "--user=daemon \\", "1");
    expect_contains(&out, "--device /dev/null:/dev/null:r \\", "1");
    expect_contains(&out, "--restart=always \\", "1");
    expect_contains(&out, "--add-host hostname2:127.0.0.2 \\", "1");
    expect_contains(&out, "--add-host hostname3:127.0.0.3 \\", "1");
    expect_contains(&out, "--workdir=/workdir \\", "1");
    expect_contains(&out, "--runtime=runc \\", "1");
}
#[test]
fn fixture1_tcp_port() {
    let out = runlikers(&["--pretty", "runlike_fixture1"]);
    expect_contains(&out, "-p 300 \\", "1");
}

#[test]
fn fixture1_tcp_port_with_host_port() {
    let out = runlikers(&["--pretty", "runlike_fixture1"]);
    expect_contains(&out, "-p 400:400 \\", "1");
}

#[test]
fn fixture1_expose() {
    let out = runlikers(&["--pretty", "runlike_fixture1"]);
    expect_contains(&out, "--expose=1000 \\", "1");
}

#[test]
fn fixture1_udp() {
    let out = runlikers(&["--pretty", "runlike_fixture1"]);
    expect_contains(&out, "-p 301/udp \\", "1");
}

#[test]
fn fixture1_udp_with_host_port() {
    let out = runlikers(&["--pretty", "runlike_fixture1"]);
    expect_contains(&out, "-p 503:502/udp \\", "1");
}

#[test]
fn fixture1_udp_with_host_port_and_ip() {
    let out = runlikers(&["--pretty", "runlike_fixture1"]);
    expect_contains(&out, "-p 127.0.0.1:601:600/udp \\", "1");
}

#[test]
fn fixture1_tty() {
    let out = runlikers(&["--pretty", "runlike_fixture1"]);
    expect_contains(&out, "-t \\", "1");
    let out2 = runlikers(&["--pretty", "runlike_fixture2"]);
    expect_not_contains(&out2, "-t \\", "2");
}

#[test]
fn fixture1_autoremove() {
    let out = runlikers(&["--pretty", "runlike_fixture5"]);
    expect_contains(&out, "--rm \\", "5");
}

#[test]
fn fixture1_restart_always() {
    let out = runlikers(&["--pretty", "runlike_fixture1"]);
    expect_contains(&out, "--restart=always \\", "1");
}

#[test]
fn fixture2_restart_on_failure() {
    let out = runlikers(&["--pretty", "runlike_fixture2"]);
    expect_contains(&out, "--restart=on-failure \\", "2");
}

#[test]
fn fixture3_restart_with_max() {
    let out = runlikers(&["--pretty", "runlike_fixture3"]);
    expect_contains(&out, "--restart=on-failure:3 \\", "3");
}

#[test]
fn fixture4_restart_not_present() {
    let out = runlikers(&["--pretty", "runlike_fixture4"]);
    expect_not_contains(&out, "--restart", "4");
}

#[test]
fn hostname() {
    let out = runlikers(&["--pretty", "runlike_fixture1"]);
    expect_contains(&out, "--hostname=Essos \\", "1");
    let out2 = runlikers(&["--pretty", "runlike_fixture2"]);
    expect_not_contains(&out2, "--hostname \\", "2");
}

#[test]
fn network_modes() {
    let out1 = runlikers(&["--pretty", "runlike_fixture1"]);
    expect_not_contains(&out1, "--network=host", "1-1");
    expect_not_contains(&out1, "--network=runlike_fixture_bridge", "1-2");
    let out2 = runlikers(&["--pretty", "runlike_fixture2"]);
    expect_contains(&out2, "--network=host", "2");
    let out3 = runlikers(&["--pretty", "runlike_fixture3"]);
    expect_contains(&out3, "--network=runlike_fixture_bridge", "3");
}

#[test]
fn privileged() {
    let out = runlikers(&["--pretty", "runlike_fixture1"]);
    expect_contains(&out, "--privileged \\", "1");
    let out2 = runlikers(&["--pretty", "runlike_fixture2"]);
    expect_not_contains(&out2, "--privileged \\", "2");
}

#[test]
fn extra_hosts() {
    let out = runlikers(&["--pretty", "runlike_fixture1"]);
    expect_contains(&out, "--add-host hostname2:127.0.0.2 \\", "1");
    expect_contains(&out, "--add-host hostname3:127.0.0.3 \\", "1");
    let out2 = runlikers(&["--pretty", "runlike_fixture2"]);
    expect_not_contains(&out2, "--add-host", "2");
}

#[test]
fn links() {
    let out = runlikers(&["--pretty", "runlike_fixture5"]);
    expect_contains(&out, "--link /runlike_fixture4:", "5");
    expect_contains(&out, "--link /runlike_fixture1:", "5");
}

#[test]
fn user() {
    let out = runlikers(&["--pretty", "runlike_fixture1"]);
    expect_contains(&out, "--user=daemon \\", "1");
    let out2 = runlikers(&["--pretty", "runlike_fixture2"]);
    expect_not_contains(&out2, "--user", "2");
}

#[test]
fn mac_address() {
    let out = runlikers(&["--pretty", "--inspect", "runlike_fixture4"]);
    expect_contains(&out, "--mac-address=6a:00:01:ad:d9:e0 \\", "4");
}

#[test]
fn ipv6() {
    let out = runlikers(&["--pretty", "runlike_fixture8"]);
    expect_contains(&out, "--ip6=2001:db8::42 \\", "8");
    let out2 = runlikers(&["--pretty", "runlike_fixture2"]);
    expect_not_contains(&out2, "--ip6", "2");
}

#[test]
fn cap_add() {
    let out = runlikers(&["--pretty", "runlike_fixture1"]);
    expect_contains(&out, "--cap-add=CAP_CHOWN", "1");
}

#[test]
fn devices() {
    let out = runlikers(&["--pretty", "runlike_fixture1"]);
    expect_contains(&out, "--device /dev/null:/dev/null:r \\", "1");
    let out2 = runlikers(&["--pretty", "runlike_fixture2"]);
    expect_contains(&out2, "--device /dev/null:/dev/null:rwm \\", "2");
}

#[test]
fn workdir() {
    let out = runlikers(&["--pretty", "runlike_fixture1"]);
    expect_contains(&out, "--workdir=/workdir \\", "1");
    let out2 = runlikers(&["--pretty", "runlike_fixture2"]);
    expect_not_contains(&out2, "--workdir", "2");
}

#[test]
fn runtime() {
    let out = runlikers(&["--pretty", "runlike_fixture1"]);
    expect_contains(&out, "--runtime=runc \\", "1");
}

#[test]
fn pid_mode() {
    let out = runlikers(&["--pretty", "runlike_fixture2"]);
    expect_contains(&out, "--pid host", "2");
    let out1 = runlikers(&["--pretty", "runlike_fixture1"]);
    expect_not_contains(&out1, "--pid", "1");
}

#[test]
fn cpuset() {
    let out = runlikers(&["--pretty", "runlike_fixture3"]);
    expect_contains(&out, "--cpuset-cpus=0", "3");
    expect_contains(&out, "--cpuset-mems=0", "3");
}

#[test]
fn entrypoint() {
    let out = runlikers(&["--pretty", "runlike_fixture7"]);
    expect_contains(&out, "--entrypoint /bin/bash", "7");
    let out6 = runlikers(&["--pretty", "runlike_fixture6"]);
    expect_not_contains(&out6, "--entrypoint", "6");
}

#[test]
fn starts_with_docker_run() {
    let out = runlikers(&["--pretty", "runlike_fixture1"]);
    starts_with(&out, "docker run ", "1");
}

#[test]
fn no_name() {
    let out = runlikers(&["--no-name", "runlike_fixture1"]);
    expect_not_contains(&out, "--name", "no-name");
}
