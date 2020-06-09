
#[derive(Debug)]
enum Protocol {
    HTTP,
    HTTPS,
    FTP,
}

struct UrlParser {
    protocol:Protocol,
    host: String,
    path:String,
    port:u16,
    query:String,
    url:String
}
impl UrlParser {
    fn url(_url : &str)->Self{
    
        let _protocol;
        let _host;
        let _path;
        let _port;
        let _query;
        
        UrlParser{
            protocol:_protocol,
            host: _host,
            path: _path,
            port: _port,
            query:_query,
            url: _url.to_string()

        }
    }
    fn protocol(&self)->Protocol {
        self.protocol
    }
    fn host(&self)->String {
        self.host
    }
    fn path(&self)->String{
        self.path
    }
    fn query(&self)->String {
        self.query
    }
    fn port(&self)->u16 {
        self.port
    }
    fn get_url(&self)->String {
        self.url
}

struct LookUpList {
    url_list: HashMap<String,bool>
}
//url string 을 키로 하여 이미 읽은 주소인지 여부를 저장한다. 만약 이미 읽은 주소이면 skip하고 , 안 읽었으면 읽고 읽었다고 set한다.
