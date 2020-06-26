#[allow(unused_imports)]
use crate::util::urlparser::{self, UrlParser};

#[test]
fn test_url_parsing() {

    let url = UrlParser::url("ftp://naver.com:20/path/sdf/?query=sdfasdsdfs34").unwrap();
    assert_eq!("ftp://naver.com:20/path/sdf/?query=sdfasdsdfs34",url.get_url());
    assert_eq!(urlparser::Protocol::FTP,url.protocol());
    assert_eq!("naver.com:20",url.hostname());
    assert_eq!("naver.com",url.host());
    assert_eq!(20,url.port());
    assert_eq!("/path/sdf/",url.path());
    assert_eq!("query=sdfasdsdfs34",url.query());  

    let url = UrlParser::url("https://naver.com:80/path/sdf/?query=sdfasdsdfs34").unwrap();
    assert_eq!("https://naver.com:80/path/sdf/?query=sdfasdsdfs34",url.get_url());
    assert_eq!(urlparser::Protocol::HTTPS,url.protocol());
    assert_eq!("naver.com:80",url.hostname());
    assert_eq!("naver.com",url.host());
    assert_eq!(80,url.port());
    assert_eq!("/path/sdf/",url.path());
    assert_eq!("query=sdfasdsdfs34",url.query());
    //protocol parsing
    let url = UrlParser::url("www.naver.com:80/path/sdf/?query=sdfasdsdfs34").unwrap();
    assert_eq!("www.naver.com:80/path/sdf/?query=sdfasdsdfs34",url.get_url());
    assert_eq!(urlparser::Protocol::HTTPS,url.protocol()); //cuz default https
    assert_eq!("www.naver.com:80",url.hostname());
    assert_eq!("www.naver.com",url.host());
    assert_eq!(80,url.port());
    assert_eq!("/path/sdf/",url.path());
    assert_eq!("query=sdfasdsdfs34",url.query());
    //except protocol parsing

    let url = UrlParser::url("https://naver.com/path/sdf/?query=sdfasdsdfs34").unwrap();
    assert_eq!("https://naver.com/path/sdf/?query=sdfasdsdfs34",url.get_url());
    assert_eq!(urlparser::Protocol::HTTPS,url.protocol());
    assert_eq!("naver.com",url.hostname());
    assert_eq!("naver.com",url.host());
    assert_eq!(443,url.port()); //default port
    assert_eq!("/path/sdf/",url.path());
    assert_eq!("query=sdfasdsdfs34",url.query());

    //except port parsing

    let url = UrlParser::url("naver.com/?query=sdfasdsdfs34").unwrap();
    assert_eq!("naver.com/?query=sdfasdsdfs34",url.get_url());
    assert_eq!(urlparser::Protocol::HTTPS,url.protocol());
    assert_eq!("naver.com",url.hostname());
    assert_eq!("naver.com",url.host());
    assert_eq!(443,url.port()); //default port
    assert_eq!("/",url.path());
    assert_eq!("query=sdfasdsdfs34",url.query());
    //changed path parsing
    
    let url = UrlParser::url("naver.com/path/sdrfd/rre/").unwrap();
    assert_eq!("naver.com/path/sdrfd/rre/",url.get_url());
    assert_eq!(urlparser::Protocol::HTTPS,url.protocol());
    assert_eq!("naver.com",url.hostname());
    assert_eq!("naver.com",url.host());
    assert_eq!(443,url.port()); //default port
    assert_eq!("/path/sdrfd/rre/",url.path());
    assert_eq!("",url.query());
    
    //query parsing

    let url = UrlParser::url("naver.com/").unwrap();
    assert_eq!("naver.com/",url.get_url());
    assert_eq!(urlparser::Protocol::HTTPS,url.protocol());
    assert_eq!("naver.com",url.hostname());
    assert_eq!("naver.com",url.host());
    assert_eq!(443,url.port()); //default port
    assert_eq!("/",url.path());
    assert_eq!("",url.query());

    //only host parsing
    //
    
    //fragement 
    let url = UrlParser::url("https://sdfs:sdfsdf@naver.com/#fragment").unwrap();
    assert_eq!(urlparser::Protocol::HTTPS,url.protocol());
    assert_eq!("naver.com",url.hostname());
    assert_eq!("naver.com",url.host());
    assert_eq!("sdfs:sdfsdf",url.userinfo());
    assert_eq!(443,url.port()); //default port
    assert_eq!("/",url.path());
    assert_eq!("",url.query());
    assert_eq!("fragment",url.fragment());


}
