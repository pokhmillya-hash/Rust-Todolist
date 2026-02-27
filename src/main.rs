use std::io;

struct Todo{
    name:String,
    bio:String,
    deadline:i32,
}

impl Todo{
    fn new(name:&str,bio:&str,deadline:i32)->Todo{
        Todo{
            name: String::from(name),
            bio: String::from(bio),
            deadline:deadline,
        }
    }
    fn info(&self){
        println!("=------{}------=", self.name);
        println!("{}", self.bio);
        println!("DEADLINE:  {}", self.deadline);
        println!("=--------------=");
    }
}

fn main() {
    let mut tasks: Vec<Todo> = Vec::new();
    loop {
        println!("=== Todo list ===");
        println!("1. Добавить задачу");
        println!("2. Показать все задачи");
        println!("3. Выход");
        
        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("Ничо не ясно");
        let action = action.trim();

        match action {
            "1" => {
                println!("Введи название");
                let mut name_buf = String::new();
                io::stdin()
                    .read_line(&mut name_buf).expect("Ошибка чтения");
                let name = name_buf.trim();

                println!("Введи описание");
                let mut bio_buf = String::new();
                io::stdin()
                    .read_line(&mut bio_buf)
                    .expect("Ошибка чтения био");
                let bio = bio_buf.trim();

                println!("Введи дни");
                let mut deadline_buf = String::new();
                io::stdin()
                    .read_line(&mut deadline_buf)
                    .expect("Не понятно");
                let deadline: i32 = deadline_buf.trim().parse().expect("Это не число!");

                let task = Todo::new(name,bio,deadline);
                tasks.push(task);
                println!("Задача добавлена");
            },

            "2" => {
                for task in &tasks{
                    task.info();
                }
            },

            "3" => {
                println!("Пока");
                break;
            },

            _ => println!("Не известная команда"),
        }

    }

}