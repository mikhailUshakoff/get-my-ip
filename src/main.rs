#[tokio::main]
async fn main() {
    let ip = external_ip::ConsensusBuilder::new()
       .add_sources(external_ip::get_http_sources::<external_ip::Sources>())
       .build()
       .get_consensus().await;

    println!("My IP address is: {:?}", ip.ok_or_else(|| "---"));
    let ip_all = external_ip::ConsensusBuilder::new()
       .add_sources(external_ip::get_http_sources::<external_ip::Sources>())
       .policy(external_ip::Policy::All)
       .build()
       .get_consensus().await;
    println!("My IP address is: {:?}", ip_all.ok_or_else(|| "---"));

    let ip_first = external_ip::ConsensusBuilder::new()
        .add_sources(external_ip::get_http_sources::<external_ip::Sources>())
        .policy(external_ip::Policy::First)
        .build()
        .get_consensus().await;
    println!("My IP address is: {:?}", ip_first.ok_or_else(|| "---"));
}
