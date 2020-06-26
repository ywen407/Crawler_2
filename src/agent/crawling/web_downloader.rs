use std::error::Error;
use std::io::prelude::*;
use crate::agent::agent::Agent;
use native_tls::TlsConnector;
use std::{net::{IpAddr, TcpStream}};
use std::fmt;
#[derive(Debug)]
enum CollectErr{
    Err(String)
}
impl fmt::Display for CollectErr{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self)
    }
}
impl Error for CollectErr{}

impl Agent {
     /* implement */
    /*
        coordinator로 부터 전달 받은 웹사이트의 Ip 를 ToVisitqueue에 추가한다.
        해당 웹페이지를 수집힌다.
    */
    pub fn get_html_collector(&mut self, cash_dns :Option<(&String,&IpAddr)>,path:String)->Result<Vec<u8>,Box<dyn Error>> {  
        //_cash_dns = Dnscash.history.get_key_value(host))                                                      
                let mut res;
                if let Some((host,ip)) = cash_dns{
                    let query = "GET ".to_string() +&path + &" HTTP/1.0\r\n\r\n".to_string();       
                    let query = query.as_bytes();                                                   
                                             
                    let connector = TlsConnector::new()?;
                    let stream = TcpStream::connect(ip.to_string()+&":443".to_string())?; //caching ip    
                    let mut stream = connector.connect(host,stream)?; //domain.      
                    stream.write_all(&query)?;                                              
                                                                                            
                    res = vec![];                                                           
                    stream.read_to_end(&mut res)?;                                          
                    Ok(res)  
                }else{
                    Err(Box::new(CollectErr::Err("error".to_string())))
                }                    
         }
}
