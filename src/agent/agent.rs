use native_tls::TlsConnector;
use queues::*;
use std::collections::HashMap;
use std::net::{TcpStream, IpAddr};
use std::fs;
use std::io::{Read, Write};
use std::error::Error;


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
    
    pub fn get_html_downloader(&mut self, cash_dns :Option<(&String,&IpAddr)>,path:String)->Result<(),Box<dyn Error>> {  
    //_cash_dns = Dnscash.history.get_key_value(host))                                                      
        
            if let Some((host,ip)) = cash_dns{
                let query = "GET ".to_string() +&path + &" HTTP/1.0\r\n\r\n".to_string();       
                let query = query.as_bytes();                                                   
                                         
                let connector = TlsConnector::new()?;
                let stream = TcpStream::connect(ip.to_string()+&":443".to_string())?; //caching ip    
                let mut stream = connector.connect(host,stream)?; //domain.      
                stream.write_all(&query)?;                                              
                                                                                        
                let mut res = vec![];                                                           
                stream.read_to_end(&mut res)?;                                          

                let dt =Local::now();
                
                fs::create_dir_all("./Download/")?;
                fs::File::create("./Download/".to_owned()+host + &dt.to_rfc3339()[..])?;
                fs::write("./Download/".to_owned()+host+ &dt.to_rfc3339()[..],&res)?;

                self.visited_queue.add(self.to_visit_queue.peek()?)?;
                self.to_visit_queue.remove()?;
            }
            Ok(())                 
        }
}

