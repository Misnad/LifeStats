use crate::models::Man;

#[derive(Clone, PartialEq)]
pub enum AppState {
    Name,
    Birth,
    Display(Man),
}
