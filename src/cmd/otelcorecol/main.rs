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

    let _factories = match components() {
        Ok(factories) => factories,
        Err(err) => {
            tracing::error!("failed to build components: {err:?}");
            std::process::exit(1);
        }
    };
}
