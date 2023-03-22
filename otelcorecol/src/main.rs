mod components;

use components::components;

fn init_subscriber() {
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_file(true)
        .with_line_number(true)
        .init()
}

#[tokio::main]
async fn main() {
    init_subscriber();

    let factories = match components() {
        Ok(factories) => factories,
        Err(err) => {
            tracing::error!("failed to build components: {err:?}");
            std::process::exit(1);
        }
    };

    let build_info = component::BuildInfo {
        command: "otelcorecol".into(),
        description: "Local OpentTelemetry Collector binary, testing only".into(),
        version: "latest".into(),
    };

    let params = otelcol::CollectorSettings {
        factories,
        build_info,
        ..Default::default()
    };

    if let Err(err) = run(params).await {
        tracing::error!("collector server run finished with error: {err:?}");
        std::process::exit(1);
    }
}

#[cfg(target_family = "unix")]
async fn run(params: otelcol::CollectorSettings) -> anyhow::Result<()> {
    runInteractive(params).await
}

#[cfg(target_family = "windows")]
async fn run(params: otelcol::CollectorSettings) -> anyhow::Result<()> {
    unimplemented!()
}

async fn runInteractive(params: otelcol::CollectorSettings) -> anyhow::Result<()> {
    // let cmd = otelcol::new_command(params);
    //cmd.execute()
    Ok(())
}
