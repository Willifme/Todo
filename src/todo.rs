extern crate chrono;

use chrono::{DateTime, Local};
use std::cmp::{Ord, Ordering};
use std::fmt;

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Priority {
    High,
    Middle,
    Low,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let prior_char = match *self {
            Priority::High => 'H',
            Priority::Middle => 'M',
            Priority::Low => 'L',
        };

        write!(f, "{}", prior_char)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq, PartialOrd)]
#[serde(rename_all = "PascalCase")]
pub struct Todo {
    pub priority: Priority,
    pub completed: bool,
    pub body: String,

    // Unfortunately, Date does not support deserialisation, so use DateTime instead
    pub end: Option<DateTime<Local>>,
}

impl Todo {
    pub fn new(
        priority: Priority,
        completed: bool,
        body: String,
        end: Option<DateTime<Local>>,
    ) -> Todo {
        Todo {
            priority,
            completed,
            body,
            end,
        }
    }

    pub fn is_overdue(&self) -> Option<bool> {
        match self.end {
            Some(end) => Some(end < Local::now()),
            None => None,
        }
    }
}

impl Ord for Todo {
    /*
     * For cmp, we check several branches:
     *   1. If both todos have a end date
     *   2. If the left todo has an end date and right doesn't
     *   3. If the right todo has an end date and left doesn't
     *   4. If neither has an end date
     *
     *   This system is done to prioritse higher priorities.
     *   Then, we prioritise the end date. The sooner, the higher.
     *   So, a todo with medium with a sooner date will be below a high todo with a later date
     *   If no date is present, then we just compare the priorities
     *
     *   Note: Priority takes precedence over end date.
     */
    fn cmp(&self, other: &Todo) -> Ordering {
        match (self.end, other.end) {
            (Some(self_end), Some(other_end)) => {
                (&self.priority, &self_end).cmp(&(&other.priority, &other_end))
            }

            // These Ordering::Less and Ordering::Greater might seem backwards but they aren't (trust me)
            (Some(_), _) => {
                (&self.priority, Ordering::Less).cmp(&(&other.priority, Ordering::Greater))
            }

            (_, Some(_)) => {
                (&self.priority, Ordering::Greater).cmp(&(&other.priority, Ordering::Less))
            }

            (_, _) => self.priority.cmp(&other.priority),
        }
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let end = match self.end {
            Some(date) => date.format("%d/%m/%Y").to_string(),
            None => "N/A".to_owned(),
        };

        write!(
            f,
            "[{}] {} - {} - {}",
            self.priority, self.completed, self.body, end
        )
    }
}
