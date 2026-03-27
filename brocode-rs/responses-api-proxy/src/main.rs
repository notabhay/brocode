use brocode_responses_api_proxy::Args as ResponsesApiProxyArgs;
use clap::Parser;

#[ctor::ctor]
fn pre_main() {
    brocode_process_hardening::pre_main_hardening();
}

pub fn main() -> anyhow::Result<()> {
    let args = ResponsesApiProxyArgs::parse();
    brocode_responses_api_proxy::run_main(args)
}
