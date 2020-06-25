use std::collections::HashMap;
use std::net::ToSocketAddrs;
use std::net::IpAddr;
use std::io::Error;

pub struct DnsCache {
        pub history: HashMap<String,IpAddr>
    }
impl DnsCache {
    pub fn new()->Self {
            DnsCache {
                history: HashMap::<String,IpAddr>::new()
            }
        }
    pub fn caching(&mut self,host: String)->Result<(),Error> { //첫번쨰로 search 된 dns ip로 맵핑
            let host_port = (&host[..],0);
            let mut ip_iter = host_port.to_socket_addrs()?;
        
            Ok(if let Some(ip_port) = ip_iter.next() {
                if !self.history.contains_key(&host){
                    self.history.insert(host,ip_port.ip());
                }
            })
        }
}
//Frontier 부분에서 맵핑을 해서 해당 맵핑(host,ip)을 agent에게 전달해줘야 하기때문에 Frontier 모듈에 넣음

