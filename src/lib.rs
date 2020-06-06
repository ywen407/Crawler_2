extern crate hyper;

use hyper::Client;
use hyper::Uri;
use hyper_tls::HttpsConnector;

extern crate queues;
use queues::*;
use std::collections::HashMap;

use std::fs;

use chrono::prelude::*;

pub struct Frontier {
    total_to_visit: Queue<Uri>
}

impl Frontier {
    pub fn new() -> Self { Self { total_to_visit: Queue::<Uri>::new() } }

    pub fn set_seed_url(&mut self, url:&str){ //초기 seed url queue에 추가
        let url = url.parse::<Uri>().unwrap();
        self.total_to_visit.add(url).unwrap();
    
    }
    
    pub fn send_to_agent(&mut self,agent_queue:&mut Queue<Uri>){ //agent queue로 추가 //
        agent_queue.add(self.total_to_visit.peek().expect("add url to agent in front")).unwrap();
        self.total_to_visit.remove().expect("totoal remove error");
    }
    /*  pub fn send_to_agent_ip(&self,agent_queue: &mut Queue<Uri>) {
        let host=self.total_to_visit.peek().unwrap();
        let host = host.to_string();
        let host_port = (&host[..],0);                                                 
        let ip_iter = host_port.to_socket_addrs().expect("socket");                                                                          
        for ip_port in ip_iter {                                                                                                   
            agent_queue.add(ip_port.ip()).expect("insert");
                          
    }
    */
}

pub struct Agent {
    pub to_visit_queue : Queue<Uri>,
    pub visited_queue : Queue<Uri>,
    pub to_send_queue : Queue<Uri>,
    pub contents_table : HashMap<String,String>, //<IpAddr,u128>?
}

#[allow(dead_code)]
impl Agent {
    fn new() -> Self {
        Self { to_visit_queue: Queue::<Uri>::new(), visited_queue :Queue::<Uri>::new(), to_send_queue :Queue::<Uri>::new(), contents_table : HashMap::<String, String>::new() } 
    }

    
    async fn get_html_downloader(&mut self)->Result<(),Box<dyn std::error::Error + Send + Sync>>{
        
        let https = HttpsConnector::new();  //about only https
        let client = Client::builder()
        .build::<_, hyper::Body>(https);
        let resp = client.get(self.to_visit_queue.peek()?.clone()).await?;
        let contents = hyper::body::to_bytes(resp.into_body()).await?;
        println!("{:?}",contents);
        let dt =Local::now();
            
        fs::create_dir_all("./Download/")?;
        fs::File::create("./Download/naver".to_owned() + &dt.to_rfc3339()[..])?;
        fs::write("./Download/naver".to_owned() + &dt.to_rfc3339()[..],contents).expect("file write error");

        self.visited_queue.add(self.to_visit_queue.peek()?)?;
        self.to_visit_queue.remove()?;
        
        Ok(())
    }
    
}
#[cfg(test)]
mod tests {
    
    use super::*;
  
    #[tokio::test]
    async fn test()->Result<(),Box<dyn std::error::Error + Send + Sync>>{
        let mut frontier = Frontier::new();
        let mut agent = Agent::new();
        
        frontier.set_seed_url("https://www.naver.com");
        frontier.send_to_agent(&mut agent.to_visit_queue);
        
        agent.get_html_downloader().await?;
        assert_eq!(r#"test"#,"test");
        Ok(())
        
    }

    
}
 