#[allow(unused_imports)]
use crate::{frontier::{dns_cashe::DnsCache, coordinator::Coordinator}, agent::agent::Agent};

#[test]
fn test_get_html_download(){

    let mut coordinator = Coordinator::new();
    let mut agent = Agent::new();
    let mut dnsmap = DnsCache::new();
    let seed ="naver.com";
    dnsmap.caching( seed.to_string()).unwrap();

    coordinator.set_seed_url(seed);
    coordinator.send_to_agent(&mut agent.to_visit_queue);

    let result = agent.get_html_downloader(dnsmap.history.get_key_value(seed),"/".to_string()).unwrap();
    
    assert_eq!(result,());
}

