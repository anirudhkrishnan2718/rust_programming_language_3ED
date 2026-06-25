use trpl::{Either, Html};

// async indicates that the function can be paused by the concurrency runtime
async fn page_title(url: &str) -> (&str, Option<String>) {
    // await indicates that the function has to wait for some future
    let response_text = trpl::get(url).await.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html());

    // function returns the url input as well, in case there is no page title to show
    (url, title)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // blocks the current thread until the future inside has completed
    trpl::block_on(async {
        // the first element is the always the name of the command
        let title_fut1 = page_title(&args[1]);
        let title_fut2 = page_title(&args[2]);

        // returns left or right future, whichever finishes first
        let (url, maybe_title) = match trpl::select(title_fut1, title_fut2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title was {title}"),
            None => println!("It had no title"),
        }
    })
}
