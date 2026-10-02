use crate::models::db::models::CreateReviewDTO;
use regex::regex;

/// Scans review for spam reviews
pub fn is_spam(payload: &CreateReviewDTO) -> bool {
    regex!(r"").is_match(payload.content.as_str())
}
