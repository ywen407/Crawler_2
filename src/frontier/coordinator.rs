use queues::*;
pub struct Coordinator {
        pub total_to_visit: Queue<String>
    }

impl Coordinator {
    pub fn new() -> Self { 
            Self { 
                total_to_visit: Queue::<String>::new()
            } 
        }

    pub fn set_seed_url(&mut self, url:&str){ //초기 seed url queue에 추가
            let url = url.parse::<String>().unwrap();
            self.total_to_visit.add(url).unwrap();
        
        }
    
    pub fn send_to_agent(&mut self,agent_queue:&mut Queue<String>){ //agent queue로 추가 //
            agent_queue.add(self.total_to_visit.peek().expect("add url to agent in front")).unwrap();
            self.total_to_visit.remove().expect("totoal remove error");
        }

}
