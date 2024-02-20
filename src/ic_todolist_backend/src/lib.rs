use std::cell::RefCell;
use candid::CandidType;

thread_local! {
    static TODOS: RefCell<Vec<Todo>> = RefCell::new(Vec::new());
}

#[derive(Clone, CandidType)]
struct Todo {
    contents: String,
    is_completed: bool,
}

#[ic_cdk::query]
fn get(idx: u64) -> Todo {
    TODOS.with(|todos| {
        let todos = todos.borrow();
        todos[idx as usize].clone()
    })
}

#[ic_cdk::update]
fn add(contents: String) -> u64 {
    let val = Todo { contents, is_completed: false };
    TODOS.with(|todos| {
        let mut todos = todos.borrow_mut();
        todos.push(val);
        todos.len() as u64 - 1
    })
}

#[ic_cdk::update]
fn update_status(idx: u64, is_completed: bool) {
    TODOS.with(|todos| {
        let mut todos = todos.borrow_mut();
        todos[idx as usize].is_completed = is_completed;
    });
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