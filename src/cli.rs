use app_dirs::{app_root, AppDataType, AppInfo};
use config::{Config, File};
use csv::Reader;
use std::collections::HashMap;
use todo::Todo;
use todos::Todos;

#[derive(Default, Clone)]
pub struct CLI {
    config: HashMap<String, String>,
}

impl CLI {
    pub fn new() -> CLI {
        CLI {
            config: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        let matches = clap_app!(todo =>
            (name: crate_name!())
            (version: crate_version!())
            (author: crate_authors!("\n"))
            (about: crate_description!())
            (@subcommand overdue =>
                (about: "shows overdue todos")
            )
            (@subcommand soon =>
                (about: "shows todos which are due soon")
            )
        ).get_matches();

        match matches.subcommand_name() {
            Some("overdue") => {
                self.load_config();

                println!("{}", self.load_todos().overdue());
            }
            Some("soon") => {
                self.load_config();

                println!("{}", self.load_todos().soon());
            }
            _ => {}
        };
    }

    fn load_config(&mut self) {
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

        self.config = settings
            .try_into::<HashMap<String, String>>()
            .unwrap_or_else(|err| panic!("Error: {}", err));
    }

    fn load_todos(&self) -> Todos {
        let mut reader = Reader::from_path(&*self.config["todos-path"]).unwrap();

        let todos_vec = reader
            .deserialize()
            .map(|record| record.unwrap_or_else(|err| panic!("Error: {}", err)))
            .collect::<Vec<Todo>>();

        Todos::new(todos_vec)
    }
}
