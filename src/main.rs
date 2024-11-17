use std::vec;

use bevy::prelude::*;
use ureq;

//<>
#[derive(Resource)]
struct UrlsToRequest(Vec<String>);

#[derive(Resource)]
struct Counter(i32);

fn main() {
    let mut app = App::new();
    app.add_systems(Update, update);
    app.insert_resource(UrlsToRequest(vec![
        "https://docs.kernel.org/core-api/index.html".to_string(),
    ]));
    app.insert_resource(Counter(0));
    app.run();
}

fn update(
    mut urls: ResMut<UrlsToRequest>,
    mut counter: ResMut<Counter>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    let mut new_urls = vec![];
    for url in urls.0.iter() {
        let mut res_urls = request_url(url);

        new_urls.append(&mut res_urls);
    }
    urls.0.clear();
    urls.0.append(&mut new_urls);

    counter.0 += 1;
    if counter.0 > 10 {
        app_exit_events.send(AppExit::Success);
    }
}

fn request_url(url: &str) -> Vec<String> {
    let req = ureq::get(url).call();

    match req {
        Ok(response) => {
            if response.status() == 200 {
                let body = response.into_string().unwrap();
                match roxmltree::Document::parse_with_options(
                    &body,
                    roxmltree::ParsingOptions {
                        allow_dtd: true,
                        nodes_limit: u32::MAX,
                    },
                ) {
                    Ok(doc) => {
                        let new_urls: Vec<String> = doc
                            .descendants()
                            .filter(|n| n.has_attribute("href"))
                            .map(|n| n.attribute("href").unwrap().to_string())
                            .collect();

                        //print urls
                        for url in new_urls.iter() {
                            println!("{}", url);
                        }

                        return new_urls;
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        println!("Body: {}", body);
                    }
                }
            } else {
                println!("Error: {}", response.status());
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    };
    return vec![];
}
