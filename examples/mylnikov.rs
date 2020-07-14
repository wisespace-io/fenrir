use async_std::task;
use fenrir::api::*;
use fenrir::mylnikov::api::*;

fn main() -> Result<(), surf::Exception> {
    task::block_on(async {
        let mylnikov: Mylnikov = Fenrir::new(None);
        let search_response = mylnikov.search_bssid("00:00:00:00:00:00").await?;
        dbg!(search_response);

        Ok(())
    })
}
