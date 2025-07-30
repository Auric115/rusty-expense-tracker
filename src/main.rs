use chrono::Utc;
use std::env;
use std::fs::{File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process;

#[derive(Debug, Clone)]
struct Expense {
    id: i32,
    date: String,
    description: String,
    amount: f32,
}

struct ExpenseTracker {
    expenses: Vec<Expense>,
    next_id: i32,
    file_name: String,
}

impl ExpenseTracker {
    fn new() -> Self {
        let mut tracker = ExpenseTracker {
            expenses: Vec::new(),
            next_id: 1,
            file_name: "expenses.txt".to_string(),
        };
        tracker.load_expenses();
        tracker
    }

    fn get_current_date() -> String {
        Utc::now().format("%Y-%m-%d").to_string()
    }

    fn load_expenses(&mut self) {
        if !Path::new(&self.file_name).exists() {
            return;
        }

        let file = File::open(&self.file_name).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines().flatten() {
            let parts: Vec<&str> = line.trim().splitn(3, ' ').collect();
            if parts.len() < 3 {
                continue;
            }

            let id_date = parts[0..2].to_vec();
            let rest = parts[2];
            let desc_split: Vec<&str> = rest.rsplitn(2, '|').collect();
            if desc_split.len() != 2 {
                continue;
            }

            if let (Ok(id), Ok(amount)) = (id_date[0].parse::<i32>(), desc_split[0].parse::<f32>()) {
                self.expenses.push(Expense {
                    id,
                    date: id_date[1].to_string(),
                    description: desc_split[1].trim().to_string(),
                    amount,
                });
                if id >= self.next_id {
                    self.next_id = id + 1;
                }
            }
        }
    }

    fn save_expenses(&self) {
        let mut file = match File::create(&self.file_name) {
            Ok(f) => f,
            Err(_) => return,
        };

        for expense in &self.expenses {
            let _ = writeln!(
                file,
                "{} {} {}|{}",
                expense.id, expense.date, expense.description, expense.amount
            );
        }
    }

    fn add_expense(&mut self, description: String, amount: f32) {
        let expense = Expense {
            id: self.next_id,
            date: Self::get_current_date(),
            description,
            amount,
        };
        self.expenses.push(expense.clone());
        self.next_id += 1;

        println!(
            "# Expense added successfully (ID: {})",
            expense.id
        );
    }

    fn list_expenses(&self) {
        if self.expenses.is_empty() {
            println!("# No expenses to display.");
            return;
        }

        println!(
            "# {:>6}{:>12}{:>18}{:>14}",
            "ID", "Date", "Description", "Amount"
        );
        for e in &self.expenses {
            println!(
                "# {:>6}{:>12}{:>18}${:>12.2}",
                e.id, e.date, e.description, e.amount
            );
        }
    }

    fn sum_expenses(&self, month: Option<u32>) -> f32 {
        let mut total = 0.0;
        for e in &self.expenses {
            let expense_month = e
                .date
                .get(5..7)
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(0);

            if month.is_none() || month.unwrap() == expense_month {
                total += e.amount;
            }
        }

        if let Some(m) = month {
            println!(
                "# Total expenses for month {}: ${:.2}",
                m, total
            );
        } else {
            println!("# Total expenses: ${:.2}", total);
        }

        total
    }

    fn delete_expense(&mut self, id: i32) {
        if let Some(pos) = self.expenses.iter().position(|x| x.id == id) {
            self.expenses.remove(pos);
            println!("# Expense deleted successfully");
        } else {
            println!("# ERROR: Expense with ID {} not found.", id);
        }
    }
}

impl Drop for ExpenseTracker {
    fn drop(&mut self) {
        self.save_expenses();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("ERROR 0x00: Insufficient Arguments.");
        process::exit(1);
    }

    let command = &args[1];
    let mut tracker = ExpenseTracker::new();

    match command.as_str() {
        "add" => {
            let mut description = String::new();
            let mut amount = 0.0;

            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "--description" if i + 1 < args.len() => {
                        description = args[i + 1].clone();
                        i += 1;
                    }
                    "--amount" if i + 1 < args.len() => {
                        amount = args[i + 1].parse().unwrap_or(0.0);
                        i += 1;
                    }
                    _ => {}
                }
                i += 1;
            }

            if description.is_empty() || amount <= 0.0 {
                eprintln!("ERROR 0x01: Invalid arguments for adding an expense.");
                process::exit(1);
            }

            tracker.add_expense(description, amount);
        }
        "list" => {
            tracker.list_expenses();
        }
        "summary" => {
            let mut month: Option<u32> = None;

            let mut i = 2;
            while i < args.len() {
                if args[i] == "--month" && i + 1 < args.len() {
                    month = args[i + 1].parse().ok();
                    i += 1;
                }
                i += 1;
            }

            tracker.sum_expenses(month);
        }
        "delete" => {
            let mut id = 0;

            let mut i = 2;
            while i < args.len() {
                if args[i] == "--id" && i + 1 < args.len() {
                    id = args[i + 1].parse().unwrap_or(0);
                    i += 1;
                }
                i += 1;
            }

            if id <= 0 {
                eprintln!("ERROR 0x02: Invalid ID for deletion.");
                process::exit(1);
            }

            tracker.delete_expense(id);
        }
        _ => {
            eprintln!("ERROR 0x03: Unknown command.");
            process::exit(1);
        }
    }
}
