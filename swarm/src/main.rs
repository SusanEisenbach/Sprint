use libra_config::config::{NodeConfig, RoleType};
use libra_swarm::{client, swarm::LibraSwarm};
use libra_tools::tempdir::TempPath;
use std::path::Path;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "Libra swarm to start local nodes")]
struct Args {
    /// Number of nodes to start (1 by default)
    #[structopt(short = "n", long, default_value = "1")]
    pub num_nodes: usize,
    /// Enable logging, by default spawned nodes will not perform logging
    #[structopt(short = "l", long)]
    pub enable_logging: bool,
    /// Start client
    #[structopt(short = "s", long)]
    pub start_client: bool,
    /// Directory used by launch_swarm to output LibraNodes' config files, logs, libradb, etc,
    /// such that user can still inspect them after exit.
    /// If unspecified, a temporary dir will be used and auto deleted.
    #[structopt(short = "c", long)]
    pub config_dir: Option<String>,

    #[structopt(short = "fk", long)]
    pub faucet_key_file_path: Option<String>,
}

fn main() {
    let args = Args::from_args();
    let num_nodes = args.num_nodes;
    let faucet_key_file_path = args.faucet_key_file_path;

    let (faucet_account_keypair, faucet_key_file_path, _temp_dir) =
        generate_keypair::load_faucet_key_or_create_default(faucet_key_file_path);

    println!(
        "Faucet account created in (loaded from) file {:?}",
        faucet_key_file_path
    );

    let mut validator_swarm = LibraSwarm::configure_swarm(
        num_nodes,
        RoleType::Validator,
        faucet_account_keypair.clone(),
        args.config_dir.clone(),
        Some("config/node.config.toml".into()), /* template_path */
        None,                                   /* upstream_config_dir */
    )
    .expect("Failed to configure validator swarm");

    validator_swarm
        .launch_attempt(RoleType::Validator, !args.enable_logging)
        .expect("Failed to launch validator swarm");

    let validator_config = NodeConfig::load(&validator_swarm.config.configs[0]).unwrap();
    let validator_set_file = validator_swarm
        .dir
        .as_ref()
        .join("0")
        .join(&validator_config.consensus.consensus_peers_file);
    println!("To run the Libra CLI client in a separate process and connect to the validator nodes you just spawned, use this command:");
    println!(
        "\tcargo run --bin client -- -a localhost -p {} -s {:?} -m {:?}",
        validator_config
            .admission_control
            .admission_control_service_port,
        validator_set_file,
        faucet_key_file_path,
    );

    let tmp_mnemonic_file = TempPath::new();
    tmp_mnemonic_file.create_as_file().unwrap();
    if args.start_client {
        let client = client::InteractiveClient::new_with_inherit_io(
            validator_swarm.get_ac_port(0),
            Path::new(&faucet_key_file_path),
            &tmp_mnemonic_file.path(),
            validator_set_file.into_os_string().into_string().unwrap(),
        );
        println!("Loading client...");
        let _output = client.output().expect("Failed to wait on child");
        println!("Exit client.");
    } else {
        // Explicitly capture CTRL-C to drop LibraSwarm.
        let (tx, rx) = std::sync::mpsc::channel();
        ctrlc::set_handler(move || {
            tx.send(())
                .expect("failed to send unit when handling CTRL-C");
        })
        .expect("failed to set CTRL-C handler");
        println!("CTRL-C to exit.");
        rx.recv()
            .expect("failed to receive unit when handling CTRL-C");
    }
    if let Some(dir) = &args.config_dir {
        println!("Please manually cleanup {:?} after inspection", dir);
    }
    println!("Exit libra-swarm.");
}
