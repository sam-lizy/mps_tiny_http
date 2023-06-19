use std::collections::VecDeque;
use std::{sync::{Arc,Mutex,Condvar}};
pub struct MessageQueue<T>{
    queue:Mutex<VecDeque<T>>,
    condar:Condvar
}
impl <T>MessageQueue<T>{
    pub fn with_capacity(capacity:usize)->Arc<MessageQueue<T>>{
        Arc::new( MessageQueue { 
            queue: Mutex::new(VecDeque::with_capacity(capacity)), 
            condar:Condvar::new()
        })
    }
    pub fn push(&self,request:T){
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(request);
        self.condar.notify_one();
    }
    pub fn pop(&self)->Option<T>{
        let mut queue = self.queue.lock().unwrap();
        loop{
            match queue.pop_front() {
                Some(value)=> return Some(value),
                None => ()
            }
            queue = self.condar.wait(queue).unwrap();
        }
    }
}
impl <T>Iterator for MessageQueue<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}
#[cfg(test)]
mod test{
    use super::*;
    use std::thread;
    use std::time::Duration;
    #[test]
    fn test_message_queue(){

        struct Serve<T>{
            queue:Arc<MessageQueue<T>>
        }
        impl <T> Iterator for Serve<T>{
            type Item = T;
            fn next(&mut self) -> Option<Self::Item> {
                self.queue.pop()
            }
        }
        let server = Serve{
            queue:MessageQueue::with_capacity(8)
        };
        let q = server.queue.clone();
        thread::spawn(move ||{
            q.push(5);
            thread::sleep(Duration::from_secs(1));
            q.push(5);
            thread::sleep(Duration::from_secs(1));
            q.push(5);
            thread::sleep(Duration::from_secs(1));
            q.push(5);
        });
        for res in server{
            println!("{}",res)
        }

    }
}