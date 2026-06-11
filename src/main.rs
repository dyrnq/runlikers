use clap::Parser;
use runlikers::Inspector;
use std::process::Command;

#[derive(Parser)]
#[command(
    name = "runlikers",
    about = "Reverse-engineer docker run command line arguments based on running containers",
    version = env!("CARGO_PKG_VERSION")
)]
struct Cli {
    /// Container name or ID
    container: Option<String>,

    /// Do not include container name in output
    #[arg(long)]
    no_name: bool,

    /// Keep the automatically assigned volume id
    #[arg(long)]
    use_volume_id: bool,

    /// Break command line into pretty lines
    #[arg(short = 'p', long)]
    pretty: bool,

    /// Read container inspect JSON from stdin
    #[arg(short = 's', long)]
    stdin: bool,

    /// Do not include labels in output
    #[arg(short = 'l', long)]
    no_labels: bool,

    /// Also emit --mount format alongside --volume
    #[arg(long)]
    mount: bool,

    /// Use docker inspect CLI instead of bollard API
    #[arg(long)]
    inspect: bool,

    /// Filter out Docker daemon defaults, only show likely user-specified options
    #[arg(long)]
    tidy: bool,

    /// Pretty indent string (default: tab, e.g. '    ' for 4 spaces)
    #[arg(long, default_value_t = String::from("\t"))]
    indent: String,

    /// Docker daemon host (e.g. unix:///var/run/docker.sock, tcp://192.168.1.100:2375, ssh://user@host)
    #[arg(short = 'H', long)]
    host: Option<String>,
}

fn inspect_via_cli(container: &str, inspector: &mut Inspector) {
    let output = Command::new("docker")
        .args(["inspect", container])
        .output()
        .unwrap_or_else(|e| {
            eprintln!("error running docker inspect: {}", e);
            std::process::exit(1);
        });
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("error: docker inspect failed: {}", stderr.trim());
        std::process::exit(1);
    }
    let json = String::from_utf8_lossy(&output.stdout);
    inspector.set_container_facts(&json).unwrap_or_else(|e| {
        eprintln!("error parsing docker inspect output: {}", e);
        std::process::exit(1);
    });
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if cli.inspect && cli.container.is_none() {
        eprintln!("error: --inspect requires a container name");
        std::process::exit(1);
    }
    if cli.container.is_none() && !cli.stdin {
        eprintln!("error: either provide a container name or use --stdin");
        std::process::exit(1);
    }

    let mut inspector = Inspector::new(cli.no_name, cli.use_volume_id, cli.pretty, cli.no_labels);
    inspector.use_mount_flag = cli.mount;
    inspector.tidy = cli.tidy;
    inspector.docker_host = cli.host;
    inspector.indent = cli.indent.clone();

    if let Some(container) = &cli.container {
        if cli.inspect {
            inspect_via_cli(container, &mut inspector);
        } else {
            inspector.inspect(container).await.unwrap_or_else(|e| {
                eprintln!("error: {}", e);
                std::process::exit(1);
            });
        }
        println!("{}", inspector.format_cli());
    } else if cli.stdin {
        use std::io::Read;
        let mut raw = String::new();
        std::io::stdin()
            .read_to_string(&mut raw)
            .unwrap_or_else(|e| {
                eprintln!("error reading stdin: {}", e);
                std::process::exit(1);
            });
        let trimmed = raw.trim();
        // Try JSON first
        if trimmed.starts_with('[') {
            // It's a docker inspect JSON array (one or more containers)
            if let Ok(serde_json::Value::Array(arr)) =
                serde_json::from_str::<serde_json::Value>(trimmed)
            {
                if arr.is_empty() {
                    eprintln!("error: empty JSON array from stdin");
                    std::process::exit(1);
                }
                // Process each container in the array
                for (i, item) in arr.iter().enumerate() {
                    let mut ins =
                        Inspector::new(cli.no_name, cli.use_volume_id, cli.pretty, cli.no_labels);
                    ins.use_mount_flag = cli.mount;
                    ins.tidy = cli.tidy;
                    ins.indent = cli.indent.clone();
                    ins.container_facts = Some(item.clone());
                    if i > 0 {
                        println!();
                    }
                    print!("{}", ins.format_cli());
                }
                return;
            }
            // fallback: try as single JSON
            inspector.set_container_facts(trimmed).unwrap_or_else(|e| {
                eprintln!("error parsing JSON: {}", e);
                std::process::exit(1);
            });
            println!("{}", inspector.format_cli());
        } else {
            // Not JSON - treat each line as a container name/ID
            let mut first = true;
            for line in trimmed.lines() {
                let name = line.trim();
                if name.is_empty() {
                    continue;
                }
                if !first {
                    println!();
                }
                first = false;
                let output = std::process::Command::new("docker")
                    .args(["inspect", name])
                    .output()
                    .unwrap_or_else(|e| {
                        eprintln!("error running docker inspect {}: {}", name, e);
                        std::process::exit(1);
                    });
                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("error: docker inspect {} failed: {}", name, stderr.trim());
                    std::process::exit(1);
                }
                let mut ins =
                    Inspector::new(cli.no_name, cli.use_volume_id, cli.pretty, cli.no_labels);
                ins.use_mount_flag = cli.mount;
                ins.tidy = cli.tidy;
                ins.indent = cli.indent.clone();
                ins.set_container_facts(&String::from_utf8_lossy(&output.stdout))
                    .unwrap_or_else(|e| {
                        eprintln!("error parsing docker inspect output: {}", e);
                        std::process::exit(1);
                    });
                print!("{}", ins.format_cli());
            }
        }
    }
}
