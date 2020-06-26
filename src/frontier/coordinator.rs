use queues::Queue;
use std::collections::hash_map::HashMap;
use crate::util::urlparser::UrlParser;

pub struct Coordinator {
    /*Frontier's Data for Crawling*/
    pub total_to_visit: Queue<UrlParser>,
    /*Frontier's Data for Converthing*/
    pub id_dic: HashMap<String,(u32,u32,u32)>, //key:URL, value:(siteID,domainID,catagoryID)
    /*Frontier's Data for Ranking*/
    pub source: Vec<u32>,
    pub destination: Vec<u32>
}


impl Coordinator {
    pub fn new() -> Self { 
        Self { 
            total_to_visit : Queue::<UrlParser>::new(),
            id_dic : HashMap::<String,(u32,u32,u32)>::new(),
            source : Vec::<u32>::new(),
            destination : Vec::<u32>::new()
        } 
    }
}