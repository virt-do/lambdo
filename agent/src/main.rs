use agent_lib::{api::service::Api, config::AgentConfig, runner_engine::service::RunnerEngine};
use anyhow::{anyhow, Result};
use clap::Parser;
use log::{debug, info, trace};

/// Agent CLI options
#[derive(Parser)]
#[clap(
    version = "0.1",
    author = "Polytech Montpellier - DevOps",
    about = "A Serverless runtime in Rust"
)]
pub struct AgentOpts {
    /// Config file path
    #[clap(short, long, default_value = "/etc/lambdo/agent/config.yaml")]
    config: String,
}

/// Main function
fn main() -> Result<()> {
    // Initialize logger
    env_logger::init();

    info!("Starting agent");

    // Parse CLI options
    let options = AgentOpts::parse();

    debug!("loading config file at {}", options.config);

    // Load config file
    let config = AgentConfig::load(options.config.as_str())?;

    trace!(
        "config file loaded successfully with content: {:#?}",
        config
    );

    // Initialize API
    let mut api = Api::new(config.serial.path, config.serial.baud_rate);

    // Send status message to serial port
    api.send_status_message()?;

    // Read request message from serial port
    let request_message = api.read_from_serial()?;

    // Initialize RunnerEngine
    let mut runner_engine = RunnerEngine::new(request_message);

    // Create the workspace
    runner_engine.create_workspace()?;

    // Run the steps of the request message
    let response_message = runner_engine.run().map_err(|e| anyhow!("{:?}", e))?;

    // Send response message to serial port
    api.send_response_message(response_message)?;

    info!("Stopping agent");
    Ok(())
}
