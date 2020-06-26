#[allow(unused_imports)]
use crate::{frontier::{coordinator::Coordinator, crawling::dns_cashe::DnsCache}, agent::agent::Agent, util::urlparser::UrlParser};

#[test]
fn replicate_url_test() {
    let mut coordinator = Coordinator::new();
    let mut agent = Agent::new();
    let mut dnsmap = DnsCache::new();
    let seed = UrlParser::url("naver.com").unwrap();
    
    dnsmap.caching( seed.clone()).unwrap();

    coordinator.set_seed_url(seed.clone()).unwrap();
    coordinator.send_to_agent(&mut agent.to_visit_queue,&mut dnsmap).unwrap();

    let result = agent.get_html_collector(dnsmap.history.get_key_value(&seed.host()),"/".to_string()).unwrap();
    let _replicate = agent.replicate_check(seed, &result).unwrap();
    assert_eq!(1,agent.contents_table.len());
}
