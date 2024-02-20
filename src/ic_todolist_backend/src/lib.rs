use std::cell::RefCell;
use candid::CandidType;

thread_local! {
    static TODOS: RefCell<Vec<Todo>> = RefCell::new(Vec::new());
}

#[derive(CandidType)]
struct Todo {
    contents: String,
    is_completed: bool,
}

#[ic_cdk::query]
fn get(_idx: u64) -> Todo {
    Todo { contents: "Dummy".to_string(), is_completed: false }
}

#[ic_cdk::update]
fn add(_contents: String) -> u64 {
    0
}

#[ic_cdk::update]
fn update_status(_is_completed: bool) {
    // dummy
}

#[cfg(test)]
mod tests {
    use super::*;
    candid::export_service!();

    #[test]
    fn gen_candid() {
        std::fs::write("ic_todolist_backend.did", __export_service()).unwrap();
    }
}