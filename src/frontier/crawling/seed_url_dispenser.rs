use crate::util::urlparser::UrlParser;
use std::net::IpAddr;
use queues::Queue;
use crate::queues::IsQueue;
use std::error::Error;
use crate::frontier::coordinator::Coordinator;
use crate::frontier::crawling::dns_cashe::DnsCache;

impl Coordinator{
     /* implement */
    /*
       먼저 기존 검색 엔진에 등록된 웹 사이트를 크롤링하여 구한 seedURL을 추가한다.
       도메인 서버로부터 seedURL의 각 URL에 해당하는 Ip들을 얻어온다.
       얻어온 Ip를 agent로 전달한다.
       마지막으로 agent가 발견한 새로운 사이트 Url들을 수집할 사이트 URL목록에 추가한다.
    */
    pub fn set_seed_url(&mut self, url:UrlParser)->Result<(), Box<dyn Error>>{ //초기 seed url queue에 추가
        
        self.total_to_visit.add(url)?;
        Ok(())
    
    }

    pub fn send_to_agent(&mut self,agent_queue:&mut Queue<IpAddr>,casher: &mut DnsCache)->Result<(),Box<dyn Error>>{ 
        let send_data = self.total_to_visit.peek()?;
        
        if !casher.history.contains_key(&send_data.host()) {
            casher.caching(send_data.clone())?;
        }
        if let Some(ip) = casher.history.get(&send_data.host()){
            agent_queue.add(*ip)?;
        }    
        self.total_to_visit.remove()?;
        Ok(())
    }
}
