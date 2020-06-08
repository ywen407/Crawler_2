use native_tls::TlsConnector;
use queues::*;
use std::collections::HashMap;
use std::net::{TcpStream, IpAddr};
use std::fs;
use std::io::{Read, Write};

use chrono::prelude::*;
#[derive(Debug)]
pub struct Agent {
        pub to_visit_queue : Queue<String>,
        pub visited_queue : Queue<String>,
        pub to_send_queue : Queue<String>,
        pub contents_table : HashMap<String,String>, //<IpAddr,u128>?
    }

#[allow(dead_code)]
impl Agent {
    pub fn new() -> Self {
            Self { 
                to_visit_queue: Queue::<String>::new(), 
                visited_queue :Queue::<String>::new(), 
                to_send_queue :Queue::<String>::new(), 
                contents_table : HashMap::<String, String>::new() 
            } 
        }
    
    pub fn get_html_downloader(&mut self, _cash_dns :Option<(&String,&IpAddr)>,path:String)->Result<(),i32> {  //_cash_dns = Dnscash.history.get_key_value(host))                                                      
        
            if let Some((host,ip)) = _cash_dns{
                let query = "GET ".to_string() +&path + &" HTTP/1.0\r\n\r\n".to_string();       
                let query = query.as_bytes();                                                   
                //println!("{:?}", String::from_utf8_lossy(&query));                              
                let connector = TlsConnector::new().unwrap();
                let stream = TcpStream::connect(ip.to_string()+&":443".to_string()).unwrap(); //caching ip    
                let mut stream = connector.connect(host,stream).unwrap(); //domain.      
                stream.write_all(&query).unwrap();                                              
                                                                                        
                let mut res = vec![];                                                           
                stream.read_to_end(&mut res).unwrap();                                          
                println!("{}", String::from_utf8_lossy(&res));  

                let dt =Local::now();
                
                fs::create_dir_all("./Download/").unwrap();
                fs::File::create("./Download/".to_owned()+host + &dt.to_rfc3339()[..]).unwrap();
                fs::write("./Download/".to_owned()+host+ &dt.to_rfc3339()[..],&res).expect("file write error");

                self.visited_queue.add(self.to_visit_queue.peek().unwrap()).unwrap();
                self.to_visit_queue.remove().unwrap();
                
                Ok(()) 
            }else{
                Err(-1) //테스트를 하기 위한 에러처리? 에러처리를 어떻게 해야될지 모르겠습니다.
            }    
                                    
            
        }
}