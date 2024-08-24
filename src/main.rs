mod content;

use content::catalog::Catalog;
use content::media::Media;

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("Syntax Code"),
    };

    let nice_movie = Media::Movie {
        title: "Inception".to_string(),
        director: "Nolan".into(),
    };

    let just_a_book = Media::Book {
        title: String::from("Not a book"),
        author: "random".to_owned(),
    };

    let podcast1 = Media::Podcast(1);

    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(nice_movie);
    catalog.add(just_a_book);
    catalog.add(podcast1);

    catalog.get_by_index(888);

    println!("The Catalog, {:#?}", catalog.items.get(200))
}
