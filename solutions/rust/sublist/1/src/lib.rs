#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_sublist<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.len() > b.len() {
        return false;
    }

    b.windows(a.len()).any(|window| window == a)
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    // todo!(
    //     "Determine if the {first_list:?} is equal to, sublist of, superlist of or unequal to {second_list:?}."
    // );V

    if first_list.len() == second_list.len() && first_list.is_empty() {
        return Comparison::Equal;
    } else if first_list.is_empty() && !second_list.is_empty() {
        return Comparison::Sublist;
    } else if !first_list.is_empty() && second_list.is_empty() {
        return Comparison::Superlist;
    }

    if first_list == second_list {
        return Comparison::Equal;
    }

    if is_sublist(first_list, second_list) {
        return Comparison::Sublist;
    }

    if is_sublist(second_list, first_list) {
        return Comparison::Superlist;
    }

    Comparison::Unequal
}
