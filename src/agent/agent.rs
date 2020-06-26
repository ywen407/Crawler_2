use md5::Digest;
use std::collections::HashSet;
use std::net::IpAddr;
use queues::Queue;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Agent {
        /*Agent's Data for Crawling */
        pub to_visit_queue : Queue<IpAddr>,  //방문할 해당 웹페이지의 ip주소.
        pub visited_queue : Queue<IpAddr>,
        pub to_send_queue : Queue<String>,
        pub contents_table : HashSet<Digest>,
        /*Agent's Data for Converting */
        pub id_dic : HashMap<String,String>,
        pub page_id_dic : HashMap<String,String>,
        /*Agent's Data for Ranking */
        pub source : Vec<u32>,
        pub destination : Vec<u32>
    }

impl Agent {
    pub fn new() -> Self {
        Self { 
            to_visit_queue: Queue::<IpAddr>::new(), 
            visited_queue : Queue::<IpAddr>::new(), 
            to_send_queue : Queue::<String>::new(), 
            contents_table : HashSet::<Digest>::new(),
            id_dic : HashMap::<String,String>::new(),
            page_id_dic : HashMap::<String,String>::new(),
            source : Vec::<u32>::new(),
            destination : Vec::<u32>::new()
        } 
    }
}
