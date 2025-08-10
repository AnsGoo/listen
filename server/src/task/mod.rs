use actix::prelude::*;
pub struct BackgroundActor;
mod music;



// 定义消息结构体
#[derive(Message)]
#[rtype(result = "()")]
pub struct TaskMessage(pub String);

impl Actor for BackgroundActor {
    type Context = Context<Self>;
}
 
impl Handler<TaskMessage> for BackgroundActor {
    type Result = ();
 
    fn handle(&mut self, msg: TaskMessage, _ctx: &mut Self::Context) {
        println!("Received message: {}", msg.0);
        music::scan_music();
    }
}