use scraper::Html;

use super::*;

#[test]
fn local_parse_restaurants() {
    let html = Html::parse_document(include_str!("tests/student-centre.html"));

    let restaurants = Restaurant::from_restaurant_list_html(&html).unwrap();

    assert_eq!(restaurants.len(), 5);

    for restaurant in &restaurants {
        eprintln!("{restaurant:#?}");
    }
}

#[tokio::test]
async fn parse_all_locations() {
    let locations = Location::all().await.unwrap();

    assert_eq!(locations.len(), 14);

    for location in locations {
        eprintln!("{location:#?}");

        let locations = location.restaurants().await.unwrap();

        for restaurant in locations {
            eprintln!("{restaurant:#?}");
        }
    }
}
