#![allow(dead_code)]

struct Post {}

impl Post {
    fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }
}

struct DraftPost {
    content: String,
}

impl DraftPost {
    fn add_content(&mut self, cnt: &str) {
        self.content.push_str(cnt);
    }

    fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    fn approve(self) -> PublishedPost {
        PublishedPost {
            content: self.content,
        }
    }

    fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}

pub struct PublishedPost {
    content: String,
}

impl PublishedPost {
    fn content(&self) -> &str {
        &self.content
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_struct() {
        let mut p = Post::new();
        p.add_content("A test for blog with struct");

        let p = p.request_review();
        let p = p.reject();
        let p = p.request_review();
        let p = p.approve();

        assert_eq!(p.content(), "A test for blog with struct");
    }
}
