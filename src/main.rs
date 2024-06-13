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

fn main() {
    //let v = vec![1, 2, 3];
    //dbg!(v);
    println!("Enter task name:");
    let name = read_string();

    println!("Enter due date:");
    let due_date = read_string();

    let task = Task{
        name,
        due_date,
        completed: true
    };

    //dbg!(&task);
    task.print();
}
