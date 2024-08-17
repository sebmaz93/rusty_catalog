enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
}

impl Media {
    fn description(&self) -> String {
        if let Media::Book { title, author } = self {
            format!("Book: {} by {}", title, author)
        } else if let Media::Movie { title, director } = self {
            format!("Movie: {} by {}", title, director)
        } else if let Media::Audiobook { title } = self {
            format!("Audiobook: {}", title)
        } else {
            String::from("media type not found.")
        }
    }
}

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
}
