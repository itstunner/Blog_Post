pub mod show{
//use std::error::Error;
use std::fmt::{Debug, Display};
#[allow(unused_variables)]

#[derive(Debug)]
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

trait <T> State<T>
where T: Display + Debug {
    fn request_review(self: Box<Self>)-> Box<dyn State>;
}
impl Post{
    pub fn new() -> Post{
        Post{
            state: Some(Box::new(Draft {})),
            content: String :: new(),
        }
    }

    pub fn add_text(&mut self, text : &str){
        self.content.push_str(text);
    } 

    pub fn content(&self) -> &str{
        ""
    }

    pub fn request_review(&mut self){
        if let Some(s) = self.state.take(){
            self.state = Some(s.request_review());
        }
    }
}

#[derive(Debug)]
struct Draft {}

#[derive(Debug)]
struct PendingReview {}

impl State for Draft{
    fn request_review(self: Box<Self>)-> Box<dyn State>{
        Box::new(PendingReview{})
        }   
    }

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State>{
        self
    }
}     
}
