use chrono::Local;
use std::fmt;
use todo::Todo;

#[derive(Deserialize, Serialize, Default)]
pub struct Todos(pub Vec<Todo>);

impl Todos {
    pub fn new(todos: Vec<Todo>) -> Todos {
        Todos(todos)
    }

    pub fn sort(&mut self) {
        self.0.sort_by(|l, r| l.cmp(r));
    }

    pub fn overdue(&self) -> Todos {
        let todos = self.0
            .iter()
            .cloned()
            .filter(|todo| todo.end.is_some())
            .filter(|todo| todo.is_overdue().unwrap())
            .collect();

        Todos(todos)
    }

    pub fn soon(&self) -> Todos {
        let todos = self.0
            .iter()
            .cloned()
            .filter(|todo| todo.end.is_some())
            .filter(|todo| {
                let end = todo.end.unwrap();

                let overdue = todo.is_overdue().unwrap();

                end >= Local::now() && !overdue
            })
            .collect();

        Todos(todos)
    }

    pub fn complete_all(&mut self) {
        self.0
            .iter()
            .cloned()
            .for_each(move |mut todo| todo.completed = true);
    }
}

impl fmt::Display for Todos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Consider cleaning this up
        if !self.0.is_empty() {
            for (i, todo) in self.0.iter().enumerate() {
                write!(f, "{} {}", i, todo)?;

                // Do not print new line if last todo
                if i < self.0.len() - 1 {
                    writeln!(f)?;
                }
            }

            Ok(())
        } else {
            write!(f, "No todos to display")
        }
    }
}
