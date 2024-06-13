use std::io;

#[derive(Debug)]
struct Task{
    name: String,
    due_date: String,
    completed: bool
}

impl Task{
    fn print(&self){
        if self.completed {print!("☑");}
        else {print!("☐");}

        println!(" {}: due {}", self.name, self.due_date);
    }
}

fn read_string() -> String{
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    return String::from(input.trim());
}

fn print_tasks(tasks: &Vec<Task>){
    println!("My tasks:");
    for task in tasks{
        task.print();
    }
}

fn add_task(tasks: &mut Vec<Task>){
    println!("Enter task name:");
    let name = read_string();

    println!("Enter due date:");
    let due_date = read_string();

    let task = Task{
        name,
        due_date,
        completed: false
    };
    tasks.push(task);
}

fn display_options(){
    println!("\n");
    println!("+: add a task");
    println!("d: display all tasks");
    println!("q: quit");
}

fn main() {
    //initialize task list
    let mut tasks: Vec<Task> = Vec::new();
    display_options();
    loop{
        println!("Please make a selection (o for all options):");
        let choice = read_string();
        match choice.trim(){
            "+" => add_task(&mut tasks),
            "d" => print_tasks(&tasks),
            "o" => display_options(),
            "q" => break,
            _ => println!("Invalid option, try again!")
        }
    }
}
