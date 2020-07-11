
use async_std::task;
use fenrir::api::*;
use fenrir::wigle::api::*;

fn main() -> Result<(), surf::Exception> {
    task::block_on(async {
        let wigle: Wigle = Fenrir::new(None);
        let response = wigle.geocode().await?;

        dbg!(response);

        Ok(())
    })
}
