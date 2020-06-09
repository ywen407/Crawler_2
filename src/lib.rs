extern crate native_tls;
extern crate queues;

pub mod frontier;
pub mod agent;
//pub mod util;



#[cfg(test)]
mod tests {

    use super::*;
    use frontier::dns_cashe::DnsCache;
    use frontier::coordinator::Coordinator;
    use agent::agent::Agent;
  
    #[test]
    fn test_caching() {
        
        let mut dnsmap = DnsCache::new();
        dnsmap.caching("naver.com".to_string()).unwrap();
        match dnsmap.history.get("naver.com"){
            Some(n) if n.to_string() == "125.209.222.141" => assert_eq!(n.to_string(),"125.209.222.141".to_string()),
            Some(n) if n.to_string() == "210.89.164.90" => assert_eq!(n.to_string(),"210.89.164.90".to_string()),
            Some(n) if n.to_string() == "210.89.160.88" => assert_eq!(n.to_string(),"210.89.160.88".to_string()), 
            Some(n) if n.to_string() == "125.209.222.142" => assert_eq!(n.to_string(),"125.209.222.142".to_string()),
            Some(_)=> println!("pass"),
            None => println!("pass"),
        }
    }
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
    
}
 
