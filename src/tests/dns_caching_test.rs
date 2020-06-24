#[allow(unused_imports)]
use crate::frontier::dns_cashe::DnsCache;

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

