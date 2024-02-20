#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        Comparison::Equal
    } else if _first_list.len() != _second_list.len() {
        Comparison::Unequal
    } else if _first_list.len() > _second_list.len() {
        Comparison::Superlist
    } else if _first_list.len() < _second_list.len() {
        Comparison::Sublist
    } else {
        Comparison::Unequal
    }
    //unimplemented!("Determine if the first list is equal to, sublist of, superlist of or unequal to the second list.");





}
