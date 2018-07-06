use app_dirs::{app_root, AppDataType, AppInfo};
use config::{Config, File};
use csv::{Reader, Writer};
use std::collections::HashMap;
use std::io;
use todo::Todo;
use todos::Todos;

#[derive(Default)]
pub struct CLI {
    #[allow(dead_code)]
    config: HashMap<String, String>,

    todos: Todos,
}

impl CLI {
    pub fn new() -> CLI {
        let config = CLI::load_config();

        CLI {
            todos: CLI::load_todos(&config["todos-path"]),
            config: config,
        }
    }

    pub fn run(&mut self) {
        let matches = clap_app!(todo =>
            (name: crate_name!())
            (version: crate_version!())
            (author: crate_authors!("\n"))
            (about: crate_description!())
            (@subcommand show =>
                (about: "show the current todos")
            )
            (@subcommand overdue =>
                (about: "shows overdue todos")
            )
            (@subcommand soon =>
                (about: "shows todos which are due soon")
            )
            (@subcommand complete =>
                (about: "complete a todo")
                (@arg todo: -t --todo +takes_value +required "complete a todo when giving the id")
            )
        ).get_matches();

        self.todos.sort();

        match matches.subcommand_name() {
            Some("show") => println!("{}", self.todos),
            Some("overdue") => println!("{}", self.todos.overdue()),
            Some("soon") => println!("{}", self.todos.soon()),
            _ => {}
        };

        if let Some(complete) = matches.subcommand_matches("complete") {
            if complete.is_present("todo") {
                let todo_id = value_t!(complete.value_of("todo"), usize)
                    .unwrap_or_else(|err| panic!("Error: {}", err));

                self.todos.0[todo_id].completed = true;

                println!("Completed: \n{}", self.todos.0[todo_id]);

                self.write_todos();
            }
        }
    }

    fn load_config() -> HashMap<String, String> {
        // Use Willifme to keep the path short
        let app_info = AppInfo {
            name: "todo",
            author: "Willifme",
        };

        let mut app_dir = app_root(AppDataType::UserConfig, &app_info)
            .unwrap_or_else(|err| panic!("Error: {}", err));

        // PathBuf::set_file_name seems to remove \todo from the path,
        // so manually push the filename instead.
        app_dir.push("config.toml");

        let mut settings = Config::new();

        // TODO: Remove this unwrap
        settings
            .merge(File::with_name(app_dir.to_str().unwrap()))
            .unwrap();

        settings
            .try_into::<HashMap<String, String>>()
            .unwrap_or_else(|err| panic!("Error: {}", err))
    }

    fn load_todos(path: &String) -> Todos {
        let mut reader = Reader::from_path(path).unwrap();

        let todos_vec = reader
            .deserialize()
            .map(|record| record.unwrap_or_else(|err| panic!("Error: {}", err)))
            .collect::<Vec<Todo>>();

        Todos::new(todos_vec)
    }

    fn write_todos(&self) {
        let mut writer = Writer::from_path(&*self.config["todos-path"])
            .unwrap_or_else(|err| panic!("Error: {}", err));

        // Writing the todos struct does not serialise correctly, so do each individually
        self.todos.0.clone().into_iter().for_each(|todo| {
            writer
                .serialize(todo)
                .unwrap_or_else(|err| panic!("Error: {}", err))
        });

        writer.flush();
    }
}
