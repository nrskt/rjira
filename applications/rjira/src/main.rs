use cli::{clap::Parser, Args, CliAdaptoer};

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let adaptor = CliAdaptoer::new(args.data());
    args.run(adaptor).await
}
