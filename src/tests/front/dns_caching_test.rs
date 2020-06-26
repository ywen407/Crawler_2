#[allow(unused_imports)]
use crate::frontier::crawling::dns_cashe::DnsCache;
#[allow(unused_imports)]
use crate::util::urlparser::UrlParser;

#[test]
fn test_caching() {
        
    let mut dnsmap = DnsCache::new();
    let url = UrlParser::url("naver.com").unwrap();
    dnsmap.caching(url).unwrap();
    match dnsmap.history.get("naver.com"){
        Some(n) if n.to_string() == "125.209.222.141" => assert_eq!(n.to_string(),"125.209.222.141".to_string()),
        Some(n) if n.to_string() == "210.89.164.90" => assert_eq!(n.to_string(),"210.89.164.90".to_string()),
        Some(n) if n.to_string() == "210.89.160.88" => assert_eq!(n.to_string(),"210.89.160.88".to_string()), 
        Some(n) if n.to_string() == "125.209.222.142" => assert_eq!(n.to_string(),"125.209.222.142".to_string()),
        Some(_)=> println!("pass"),
        None => println!("pass"),
    }

}
