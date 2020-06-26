#[allow(unused_imports)]
use crate::{frontier::{coordinator::Coordinator, crawling::dns_cashe::DnsCache}, agent::agent::Agent, util::urlparser::UrlParser};

#[test]
fn test_get_html_download(){

    let mut coordinator = Coordinator::new();
    let mut agent = Agent::new();
    let mut dnsmap = DnsCache::new();
    let seed = UrlParser::url("naver.com").unwrap();
    
    dnsmap.caching( seed.clone()).unwrap();

    coordinator.set_seed_url(seed.clone()).unwrap();
    coordinator.send_to_agent(&mut agent.to_visit_queue,&mut dnsmap).unwrap();

    let _result = agent.get_html_collector(dnsmap.history.get_key_value(&seed.host()),"/".to_string()).unwrap();

}
