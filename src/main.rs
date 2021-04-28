use std::env;

struct Todo {
    name: String,
    completed: char,
}

struct Todos {
    list: Vec<Todo>,
}

impl Todo {
    fn add(name: String) -> Todo {
        return Todo {
            name: name,
            completed: ' ',
        };
    }
}

impl Todos {
    fn add() -> Todos {
        return Todos { list: Vec::new() };
    }

    fn add_item(&mut self, name: String) {
        let todo_item = Todo::add(name);
        self.list.push(todo_item);
    }

    fn print(&self) {
        for item in &self.list {
            println!("- [{}] {}", item.completed, item.name)
        }
    }
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let command = arguments[1].clone();
    let mut todo_list = Todos::add();

    if command == "get" {
        todo_list.print()
    } else if command == "add" {
        let task = arguments[2].clone();
        todo_list.add_item(task.to_string());
        todo_list.print()
    }

    println!("{:?}", arguments)
}
