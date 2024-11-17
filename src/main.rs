use std::vec;

use bevy::{prelude::*, utils::hashbrown::HashSet};
use ureq;
use url::Url;

//<>
#[derive(Resource)]
struct UrlsToVisit(Vec<String>);

#[derive(Resource)]
struct VisitedUrls(HashSet<String>);

#[derive(Resource)]
struct Counter(i32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, update)
        .insert_resource(UrlsToVisit(vec![
            "https://docs.kernel.org/core-api/index.html".to_string(),
        ]))
        .insert_resource(VisitedUrls(HashSet::default()))
        .insert_resource(Counter(0))
        .run();
}

fn update(
    mut urls_to_visit: ResMut<UrlsToVisit>,
    mut visited_urls: ResMut<VisitedUrls>,
    mut counter: ResMut<Counter>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    let mut new_urls = vec![];
    for url in urls_to_visit.0.iter() {
        println!("Requesting: {}", url);

        let res_urls = request_url(url);

        new_urls.append(&mut res_urls.iter().map(|x| x.clone()).collect());
    }
    visited_urls.0.extend(urls_to_visit.0.clone());

    urls_to_visit.0.clear();
    urls_to_visit.0.append(&mut new_urls);

    println!("Counter: {}", counter.0);

    counter.0 += 1;
    if counter.0 > 1 {
        visited_urls.0.iter().for_each(|url| println!("{}", url));
        app_exit_events.send(AppExit::Success);
    }
}

fn request_url(url: &str) -> HashSet<String> {
    let req = ureq::get(url).call();

    match req {
        Ok(response) => {
            if response.status() == 200 {
                let body = response.into_string().unwrap();
                let link_selector = scraper::Selector::parse("[href]").unwrap();
                let document = scraper::Html::parse_document(&body);
                let elements_with_links = document.select(&link_selector);

                let mut new_urls: HashSet<String> = HashSet::default();

                for element in elements_with_links {
                    let href = element.value().attr("href").unwrap();

                    let mut parsed_url = Url::parse(url).unwrap().join(href).unwrap();
                    parsed_url.set_path("");
                    let url_string = parsed_url.to_string();
                    if !new_urls.contains(&url_string) {
                        new_urls.insert(url_string);
                    }
                }

                return new_urls;
            } else {
                println!("Error: {}", response.status());
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    };
    return HashSet::default();
}
