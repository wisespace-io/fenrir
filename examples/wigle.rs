use async_std::task;
use fenrir::api::*;
use fenrir::wigle::api::*;

fn main() -> Result<(), surf::Exception> {
    let token = std::env::var("WIGLE_TOKEN").expect("You need to give your WIGLE_TOKEN as an environment variable");
    task::block_on(async {
        let wigle: Wigle = Fenrir::new(Some(token));
        let response = wigle.geocode("1600 Amphitheatre Parkway").await?;

        dbg!(response);

        Ok(())
    })
}
