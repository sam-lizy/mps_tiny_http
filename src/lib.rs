mod queue;
mod common;
mod response;
mod request;
use queue::MessageQueue;
use std::{
    net::TcpListener,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread
};

use request::Request;

#[derive(Clone)]
pub struct ListenAddr<'a> {
    ip: &'a str,
}
impl<'a> ListenAddr<'a> {
    fn new(addr: &'a str) -> Self {
        ListenAddr { ip: addr }
    }
}
pub struct IncomingRequests<'a> {
    server: &'a Serve<'a>,
}
pub struct Serve<'a> {
    addr: ListenAddr<'a>,
    messages: Arc<MessageQueue<Request>>,
    close_trigger: Arc<AtomicBool>,
}
impl<'a> Serve<'a> {
    pub fn new(addr: &'a str) -> Self {
        let listen_addr = ListenAddr::new(addr);
        Serve {
            addr: listen_addr,
            messages: MessageQueue::with_capacity(8),
            close_trigger: Arc::new(AtomicBool::new(false)),
        }
    }
    pub fn connect(&self) {
        let tcp_listener = TcpListener::bind(self.addr.ip).unwrap();
        let message_queue = self.messages.clone();
        let tcp = tcp_listener.try_clone().unwrap();
        let inside_close_trigger = self.close_trigger.clone();
        thread::spawn(move || {
            while !inside_close_trigger.load(Ordering::Relaxed) {
                match tcp.accept() {
                    Ok((stream, _)) => {
                        let new_res = Request::new(stream).unwrap();
                        message_queue.push(new_res);
                    }
                    Err(e) => println!("{}",e),
                };
            }
        });
    }
    pub fn incoming_requests(&self)->IncomingRequests{
        IncomingRequests{
            server:self
        }
    }
    pub fn route(mut self,method:&str,task:Box<dyn Fn() + Send>)->Self{
        todo!();
    }

}
impl Iterator for IncomingRequests<'_> {
    type Item = Request;
    fn next(&mut self) -> Option<Self::Item> {
        self.server.messages.pop()
    }
}
impl Drop for Serve<'_>{
    fn drop(&mut self) {
        self.close_trigger.store(true, Ordering::Relaxed);
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_run() {
        let server = Serve::new("127.0.0.1:7878");
        server.connect();
        for mut res in server.incoming_requests(){
            println!("{}",res.method());
            println!("{}",res.url());
            res.response("6666");
        }


    }
}
