use std::convert::TryFrom;
use std::{num::ParseIntError, cmp::PartialEq};

#[derive(Debug,Copy,Clone,PartialEq)]
pub enum Protocol {
        HTTP,
        HTTPS,
        FTP,
    }

impl TryFrom<Option<&str>> for Protocol { 
    type Error = &'static str;

    fn try_from(s:Option<&str>) -> Result <Self,Self::Error> {
        match s {
            Some("http")    => Ok(Protocol::HTTP),
            Some("ftp")     => Ok(Protocol::FTP),
            Some("https")   => Ok(Protocol::HTTPS),
            None            => Ok(Protocol::HTTPS),
            _               => Err("this scheme is not valied:")
       }
    }
}
/* 파싱한 스트링을 프로토콜 타입으로 변환. 원치않는 타입조건이면 에러 */

#[derive(Debug,Clone)]
pub struct Port(u16);

impl TryFrom<Option<&str>> for Port {
    type Error = &'static str;

     fn try_from(number: Option<&str>) -> Result<Self,Self::Error> {
        match number {
            Some("20")  => Ok(Port(20)),
            Some("21")  => Ok(Port(21)),
            Some("80")  => Ok(Port(80)),
            Some("443") => Ok(Port(443)),
            None      => Ok(Port(443)),
            _         => Err("this port is not valid")
        }
    }
}
/* 파싱한 스트링을 포트 타입으로 변환. 원치않는 포트면 에러 */
#[derive(Debug,Clone)]
pub struct UrlParser {
    protocol:Protocol,
    userinfo:Option<String>,
    host: String,
    hostname: String,
    path:Option<String>,
    port:Port,
    query:Option<String>,
    fragment:Option<String>,
    url:String
}


impl UrlParser {

    pub fn url(url : &str)->Result<Self,ParseIntError>{
            let mut protocol=Protocol::HTTPS;
            let hostname; //host including port
            let mut userinfo=None;
            let mut host;
            let mut path=None;
            let mut port=Port(443);
            let mut query=None;
            let mut fragment=None;
            let mut parse_url =url.clone();
            let mut len;
            let url =url.to_string();
            
            if parse_url.contains("#") { //extract fragment
                let url_parsed:Vec<&str> = parse_url.split("#").collect();
                fragment = Some(url_parsed[1][..].to_string());
                parse_url = &url_parsed[0][..];
            }
            if parse_url.contains("//") { //extract protocol
                let url_parsed :Vec<&str> =parse_url.split("//").collect();
                len = url_parsed[0].len()-1;
                let temp =&url_parsed[0][..len];
                if temp == ""{
                    let temp = None;
                    match Protocol::try_from(temp){
                        Ok(n) => protocol=n,
                        Err(e) => println!("{}",e)
                    } 
                }else{
                    let temp = Some(temp);
                    match Protocol::try_from(temp){
                        Ok(n) => protocol=n,
                        Err(e) => println!("{}",e)
                    }
                }
                
                parse_url = &url_parsed[1][..]; 
            }
            if parse_url.contains("@") { //extract userinfo
                let url_parsed:Vec<&str> = parse_url.split("@").collect();
                userinfo = Some(url_parsed[0][..].to_string());
                parse_url = &url_parsed[1][..];
            }
            if parse_url.contains("/"){ //extract hostname if none of query, extract path. 
                let url_parsed: Vec<&str> = parse_url.split("/").collect();
                hostname= url_parsed[0][..].to_string();
                host= hostname.clone();
                len =hostname.len();
                path=Some(parse_url[len..].to_string());
                parse_url = &parse_url[len..];
            }else {
                hostname =parse_url.to_string().clone();
                host=parse_url.to_string().clone();
            }
            if parse_url.contains("?") { //extract query , path
                let url_parsed: Vec<&str> =parse_url.split("?").collect();
                path= Some(url_parsed[0][..].to_string());
                let temp = &url_parsed[1][..];
                if temp ==""{
                    query = None;       
                }else {
                    query = Some(temp.to_string());
                }
            }
            if hostname.contains(":"){ //extarct host,port
                
                let url_parsed :Vec<&str> = hostname.split(":").collect();
                host = url_parsed[0][..].to_string();
                let temp =&url_parsed[1][..];
                if temp =="" {
                    let temp =None;
                    match Port::try_from(temp) {
                        Ok(n) => port=n,
                        Err(e) => println!("{}",e)
                    }
                }else {
                    let temp =Some(temp);
                    match Port::try_from(temp) {
                        Ok(n) => port=n,
                        Err(e) => println!("{}",e)
                    }
                }
            
            }
            
            Ok(UrlParser{
                protocol,
                userinfo,
                host,
                hostname,
                path,
                port,
                query,
                fragment,
                url
            })
        }


    pub fn protocol(&self)->Protocol {
            self.protocol
        }
    pub fn userinfo(&self)->String {
            match &self.userinfo{
                Some(n)=>n.clone(),
                None=>"".to_string()
            }
        }
    pub fn host(&self)->String {
            self.host.clone()
        }
    pub fn hostname(&self)->String {
            self.hostname.clone()
        }
    pub fn path(&self)->String{
            match &self.path{
                Some(n) => n.clone(),
                None =>"/".to_string()
            }
        }
    pub fn query(&self)->String {
            match &self.query{
                Some(n)=>n.clone(),
                None=>"".to_string()
            }
        }
    pub fn port(&self)->u16 {
            self.port.0
        }
    pub fn fragment(&self)->String {
            match &self.fragment{
                Some(n)=>n.clone(),
                None   =>"".to_string()
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
