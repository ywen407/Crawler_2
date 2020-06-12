//#[derive(Debug)]

use std::{num::ParseIntError, cmp::PartialEq};

const HTTP: &str="http";
const HTTPS: &str = "https";
const FTP: &str = "ftp";

#[derive(Debug,Copy,Clone,PartialEq)]
pub enum Protocol {
        HTTP,
        HTTPS,
        FTP,
    }
//enum Protocol을 어떻게 써야 할지 잘모르겠음.
pub struct UrlParser {
    protocol:Protocol,
    host: String,
    hostname: String,
    path:String,
    port:Option<u16>,
    query:String,
    url:String
}
impl UrlParser {

    pub fn url(url : &str)->Result<Self,ParseIntError>{
            let mut protocol=Protocol::HTTPS;
            let hostname; //host including port 
            let mut host;
            let mut path="".to_string();
            let mut port=None;
            let mut query="".to_string();
            let mut i_url =url.clone();
            let mut len;
            let url =url.to_string();
            
            if i_url.contains("//") { //extarct protocol
                let url_v :Vec<&str> =i_url.split("//").collect();
                len = url_v[0].len()-1;
                let temp =&url_v[0][..len];

                protocol = if temp == "http"{
                    Protocol::HTTP
                }else if temp == "ftp" {
                    Protocol::FTP
                }else {
                    Protocol::HTTPS
                };

                i_url = &url_v[1][..]; 
            }
            
            if i_url.contains("/"){ //extract hostname if none of query, extract path. 
                let url_v: Vec<&str> = i_url.split("/").collect();
                hostname= url_v[0][..].to_string();
                host= hostname.clone();
                len =hostname.len();
                i_url = &i_url[len..];
                path = url[len..].to_string().clone();
            }else {
                hostname =i_url.to_string().clone();
                host=i_url.to_string().clone();
            }
            
            if i_url.contains("?") { //extract query , path
                let url_v: Vec<&str> =i_url.split("?").collect();
                path= url_v[0][..].to_string();
                query= url_v[1][..].to_string();
            }
            if hostname.contains(":"){ //extarct host,port
                
                let url_v :Vec<&str> = hostname.split(":").collect();
                host = url_v[0][..].to_string();
                port = Some(url_v[1][..].to_string().parse::<u16>()?);
            
            }
            
            Ok(UrlParser{
                protocol,
                host,
                hostname,
                path,
                port,
                query,
                url
            })
        }
    pub fn protocol(&self)->Protocol {
            self.protocol
        }
    pub fn host(&self)->String {
            self.host.clone()
        }
    pub fn hostname(&self)->String {
            self.hostname.clone()
        }
    pub fn path(&self)->String{
            self.path.clone()
        }
    pub fn query(&self)->String {
            self.query.clone()
        }
    pub fn port(&self)->u16 {
            match self.port{
                Some(n) => { n},
                None => {443}
            }
        }
    pub fn get_url(&self)->String {
            self.url.clone()
        }
}
/*
struct LookUpList {
    url_list: HashMap<String,bool>
}
//url string 을 키로 하여 이미 읽은 주소인지 여부를 저장한다. 만약 이미 읽은 주소이면 skip하고 , 안 읽었으면 읽고 읽었다고 set한다.
*/
