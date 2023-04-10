mod deployment;
use deployment::Monitor;

#[tokio::main()]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let monitor = Monitor::new().await?;

    monitor.start();

    loop {
        // match monitor.cockroach_exists().await {
        //     Ok(exists) => println!("database exists: {}", exists),
        //     Err(err) => println!("oops: {:?}", err)
        // };
        
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
