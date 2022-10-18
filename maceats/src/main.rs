use maceats::Location;

#[tokio::main]
async fn main() {
    let student_centre = Location::new("Student Centre");
    let restaurants = student_centre.restaurants().await.unwrap();

    for restaurant in restaurants {
        println!("{}", serde_json::to_string_pretty(&restaurant).unwrap());
    }
}
