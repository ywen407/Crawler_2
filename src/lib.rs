extern crate native_tls;
extern crate queues;

pub mod frontier;
pub mod agent;
pub mod util;


#[cfg(test)]
mod tests {

    use super::*;
    use frontier::dns_cashe::DnsCache;
    use frontier::coordinator::Coordinator;
    use agent::agent::Agent;
    use util::urlparser;
    use util::urlparser::UrlParser;
    #[test]
    fn test_url_parsing() {

        let url = UrlParser::url("ftp://naver.com:20/path/sdf/?query=sdfasdsdfs@34").unwrap();
        assert_eq!("ftp://naver.com:20/path/sdf/?query=sdfasdsdfs@34",url.get_url());
        assert_eq!(urlparser::Protocol::FTP,url.protocol());
        assert_eq!("naver.com:20",url.hostname());
        assert_eq!("naver.com",url.host());
        assert_eq!(20,url.port());
        assert_eq!("/path/sdf/",url.path());
        assert_eq!("query=sdfasdsdfs@34",url.query());  

        let url = UrlParser::url("https://naver.com:80/path/sdf/?query=sdfasdsdfs@34").unwrap();
        assert_eq!("https://naver.com:80/path/sdf/?query=sdfasdsdfs@34",url.get_url());
        assert_eq!(urlparser::Protocol::HTTPS,url.protocol());
        assert_eq!("naver.com:80",url.hostname());
        assert_eq!("naver.com",url.host());
        assert_eq!(80,url.port());
        assert_eq!("/path/sdf/",url.path());
        assert_eq!("query=sdfasdsdfs@34",url.query());
        //protocol parsing
        let url = UrlParser::url("www.naver.com:80/path/sdf/?query=sdfasdsdfs@34").unwrap();
        assert_eq!("www.naver.com:80/path/sdf/?query=sdfasdsdfs@34",url.get_url());
        assert_eq!(urlparser::Protocol::HTTPS,url.protocol()); //cuz default https
        assert_eq!("www.naver.com:80",url.hostname());
        assert_eq!("www.naver.com",url.host());
        assert_eq!(80,url.port());
        assert_eq!("/path/sdf/",url.path());
        assert_eq!("query=sdfasdsdfs@34",url.query());
        //except protocol parsing

        let url = UrlParser::url("https://naver.com/path/sdf/?query=sdfasdsdfs@34").unwrap();
        assert_eq!("https://naver.com/path/sdf/?query=sdfasdsdfs@34",url.get_url());
        assert_eq!(urlparser::Protocol::HTTPS,url.protocol());
        assert_eq!("naver.com",url.hostname());
        assert_eq!("naver.com",url.host());
        assert_eq!(443,url.port()); //default port
        assert_eq!("/path/sdf/",url.path());
        assert_eq!("query=sdfasdsdfs@34",url.query());

        //except port parsing

        let url = UrlParser::url("naver.com/?query=sdfasdsdfs@34").unwrap();
        assert_eq!("naver.com/?query=sdfasdsdfs@34",url.get_url());
        assert_eq!(urlparser::Protocol::HTTPS,url.protocol());
        assert_eq!("naver.com",url.hostname());
        assert_eq!("naver.com",url.host());
        assert_eq!(443,url.port()); //default port
        assert_eq!("/",url.path());
        assert_eq!("query=sdfasdsdfs@34",url.query());
        //changed path parsing
        
        let url = UrlParser::url("naver.com/path/sdrfd/rre/").unwrap();
        assert_eq!("naver.com/path/sdrfd/rre/",url.get_url());
        assert_eq!(urlparser::Protocol::HTTPS,url.protocol());
        assert_eq!("naver.com",url.hostname());
        assert_eq!("naver.com",url.host());
        assert_eq!(443,url.port()); //default port
        assert_eq!("/path/sdrfd/rre/",url.path());
        assert_eq!("",url.query());
        
        //query parsing

        let url = UrlParser::url("naver.com/").unwrap();
        assert_eq!("naver.com/",url.get_url());
        assert_eq!(urlparser::Protocol::HTTPS,url.protocol());
        assert_eq!("naver.com",url.hostname());
        assert_eq!("naver.com",url.host());
        assert_eq!(443,url.port()); //default port
        assert_eq!("/",url.path());
        assert_eq!("",url.query());

        //only host parsing


    }
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
 
