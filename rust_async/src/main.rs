
use futures::executor::block_on;

/* the value returned here is a future.It needs to be run on an executor for anything to happen */
async fn hello_world(){
    println!("Hello world!");
}

fn main() {
    let future = hello_world();
    block_on(future);

}
