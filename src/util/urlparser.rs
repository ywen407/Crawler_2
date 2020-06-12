//#[derive(Debug)]
/*enum Protocol {
    HTTP,
    HTTPS,
    FTP,
}
*/

pub struct UrlParser {
    protocol:String,
    host: String,
    hostname: String,
    path:String,
    port:String,
    query:String,
    url:String
}
impl UrlParser {

    pub fn url(url : &str)->Self{
            let mut i_protocol="https".to_string();
            let i_hostname; //host including port 
            let mut i_host;
            let mut i_path="".to_string();
            let mut i_port="443".to_string();
            let mut i_query="".to_string();
            let mut i_url =url.clone();
            let mut len;
            
            if i_url.contains("//") { //extarct protocol
                let url_v :Vec<&str> =i_url.split("//").collect();
                len = url_v[0].len()-1;
                i_protocol= url_v[0][..len].to_string();
                i_url = &url_v[1][..]; 
            }
            
            if i_url.contains("/"){ //extract hostname if none of query, extract path. 
                let url_v: Vec<&str> = i_url.split("/").collect();
                i_hostname= url_v[0][..].to_string();
                i_host= i_hostname.clone();
                len =i_hostname.len();
                i_url = &i_url[len..];
                i_path = url[len..].to_string().clone();
            }else {
                i_hostname =i_url.to_string().clone();
                i_host=i_url.to_string().clone();
            }
            
            if i_url.contains("?") { //extract query , path
                let url_v: Vec<&str> =i_url.split("?").collect();
                i_path= url_v[0][..].to_string();
                i_query= url_v[1][..].to_string();
            }
            if i_hostname.contains(":"){ //extarct host,port
                
                let url_v :Vec<&str> = i_hostname.split(":").collect();
                i_host = url_v[0][..].to_string();
                i_port = url_v[1][..].to_string();
            
            }
    
            UrlParser{
                protocol:i_protocol,
                host: i_host,
                hostname: i_hostname,
                path: i_path,
                port: i_port,
                query:i_query,
                url: url.to_string()

            }
        }
    pub fn protocol(&self)->&String {
            &self.protocol
        }
    pub fn host(&self)->&String {
            &self.host
        }
    pub fn hostname(&self)->&String {
            &self.hostname
        }
    pub fn path(&self)->&String{
            &self.path
        }
    pub fn query(&self)->&String {
            &self.query
        }
    pub fn port(&self)->&String {
            &self.port
        }
    pub fn get_url(&self)->&String {
            &self.url
        }
}
/*
struct LookUpList {
    url_list: HashMap<String,bool>
}
//url string 을 키로 하여 이미 읽은 주소인지 여부를 저장한다. 만약 이미 읽은 주소이면 skip하고 , 안 읽었으면 읽고 읽었다고 set한다.
*/

