use rusqlite::{params, Connection, Result};
use std::io;
use std::io::{Read, Write};
use std::collections::HashMap;
use std::process::Command;
use cocoon::MiniCocoon;

#[derive(Debug)]
struct Password {
    id: i32,
    service_id: i32,
    username: String,
    password: String
}

#[derive(Debug)]
struct Service {
    id: i32,
    service_name: String,
}

trait ExecuteQueries {
    fn get_services(&self) -> HashMap<i32, String>;
    fn get_passwords(&self);
    fn add_service(&self);
    fn add_password(&self);

}

impl ExecuteQueries for Connection {
    fn get_services(&self) -> HashMap<i32, String> {
        let statement = &mut self.prepare("SELECT id, service_name FROM services").expect("Failed to prepare query");
        let service_iter = statement.query_map([], |row| {
            Ok(Service {
                id: row.get(0)?,
                service_name: row.get(1)?
            })
        }).expect("Failed to execute query");

        let mut service_names: HashMap<i32, String> = HashMap::new(); // &str потому что не меняем, и наверное это будет более эффективно в плане памяти

        println!(":: Выберите сервис:");
        for s in service_iter {
            let data = s.unwrap();
            println!("{:?}. {:?}", data.id, data.service_name);
            service_names.insert(data.id, data.service_name);
        }

        service_names
    }

    fn get_passwords(&self) {
        let service_names = self.get_services();

        // let input = get_input().parse::<i32>().unwrap_or(0);
        let input = i32::get_input();

        let statement = &mut self.prepare("SELECT * FROM passwords WHERE service_id = ?").expect("Failed to prepare query");
        let password_iter = statement.query_map(&[&input], |row| {
            Ok(Password {
                id: row.get(0)?,
                service_id: row.get(1)?,
                username: row.get(2)?,
                password: row.get(3)?
            })
        }).expect("Failed to execute query");

        println!(":: Пароли для сервиса {}:", service_names[&input]);
        for p in password_iter {
            let data = p.unwrap();
            println!("ID: {}, Username: {}, Password: {}", data.id, data.username, data.password);
        }

    }

    fn add_service(&self) {
        println!("Введите название нового сервиса:");
        let service_name: String = String::get_input();

        // &self.execute(
        //     "INSERT INTO services(service_name) VALUES (?)",
        //     params![service_name]
        // );

        let statement = &mut self.prepare("INSERT INTO services(service_name) VALUES (?)").expect("Failed to prepare query");
        if let Ok(_result) = statement.execute(params![service_name]) {
            println!("✅ :: Добавлен сервис `{service_name}`");
        }

        // Ok(())
    }

    fn add_password(&self) {
        // service_id: i32, username: String, password: String
        let service_names = self.get_services();
        let service_id: i32 = i32::get_input();

        println!("Введите имя пользователя для сервиса {}:", service_names[&service_id]);
        let username: String = String::get_input();

        println!("Отлично. Теперь введите пароль для пользователя {username} в сервисе {}:", service_names[&service_id]);
        let password: String = String::get_input();

        // &self.execute(
        //     "INSERT INTO passwords(service_id, username, password) VALUES (?1, ?2, ?3)",
        //     params![service_id, username, password]
        // ); // error[E0277]: the `?` operator can only be used in a method that returns `Result` or `Option` (or another type that implements `FromResidual`)

        let statement = &mut self.prepare("INSERT INTO passwords(service_id, username, password) VALUES (?1, ?2, ?3)").expect("Failed to prepare query");

        if let Ok(_result) = statement.execute(params![service_id, username, password]) {
            println!("✅ :: Добавлена учетная запись пользователя `{username}` для сервиса `{}`", service_names[&service_id]);
        }

        // Ok(())
    }
}

fn main() -> Result<()> {
    let conn = Connection::open("pwds.db")?;

    conn.execute(
    "create table if not exists services(\
            id integer primary key, \
            service_name text not null
        )",
        ()
    )?;

    conn.execute(
    "create table if not exists passwords(\
            id integer primary key, \
            service_id integer references services(id), \
            username text, \
            password text\
        )",
        ()
    )?;

    // conn.add_service("Twitter".to_string());
    // conn.add_service("Google".to_string());

    // conn.add_password(2, "test".to_string(), "test".to_string());

    // return Ok(());

    // TODO: make menu as separate function

    // let mut pwd = "my_secret_password".to_owned().into_bytes();
    // let mut cocoon = MiniCocoon::from_key(b"0123456789qwertyuioplkjhgfdsazxcvbnm", &[0; 32]);

    // let detached_prefix = cocoon.encrypt(&mut pwd);
    // println!("{}", &pwd);
    // println!("{}", &detached_prefix);

    // println!("Выберите опцию:");
    // println!("1. Посмотреть пароли");
    // println!("2. Добавить сервис");
    // println!("3. Добавить пароль");
    // print!("> ");

    // io::stdout().flush().unwrap();

    // let mut buffer = String::new();
    // io::stdin().read_line(&mut buffer);

    // let input = buffer.trim();
    // let input = String::get_input();
    //
    // // TODO: why i should use 'as_str()'?
    // match input.as_str() {
    //     "1" => conn.get_passwords(),
    //     // "2" => conn.add_password(),
    //     // "3" => conn.add_service(),
    //     _ => {
    //         println!("Я такого не умею");
    //     }
    // }

    loop {
        // Press any key to continue, then flush (doesnt work)
        // let _ = Command::new("pause").status();
        // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        println!("Выберите опцию:");
        println!("1. Посмотреть пароли");
        println!("2. Добавить сервис");
        println!("3. Добавить пароль");
        println!();
        println!("e. Выход");

        let input = String::get_input();
        match input.trim() {
            "1" => conn.get_passwords(),
            "2" => conn.add_service(),
            "3" => conn.add_password(),
            "e" => {
                println!("Выход из приложения..");
                break;
            }
            _ => {
                println!("Функция не поддерживается приложением");
                break;
            }
        }
    }


    Ok(())
}

trait GetInput {
    fn get_input() -> Self;
}

impl GetInput for String {
    fn get_input() -> Self {
        let mut buffer = String::new();
        loop {
            print!("> ");
            if let Err(error) = io::stdout().flush() {
                panic!("Error occured: {error}");
            }

            if let Err(error) = io::stdin().read_line(&mut buffer) {
                panic!("Failed to read input: {error}");
            }

            let value: String = buffer.trim().to_string();

            if value.len() < 1 {
                buffer.clear();
                continue;
            }

            return buffer.trim().to_string();
        }
    }
}

impl GetInput for i32 {
    fn get_input() -> Self {
        let mut buffer = String::new();
        loop {
            print!("> ");

            if let Err(error) = io::stdout().flush() {
                panic!("Error occurred: {error}");
            }

            // io::stdin().read_line(&mut buffer).expect("Failed to read input");
            if let Err(error) = io::stdin().read_line(&mut buffer) {
                panic!("Failed to read input. {error}");
            }


            let input = buffer.trim();

            // return input.parse::<i32>().expect("Unable to parse i32 from given input");
            if let Ok(value) = input.parse::<i32>() {
                return value;
            }

            println!("Unable to parse i32 from given input. Please provide correct input:");
            buffer.clear();
        }

    }
}

/*fn get_input() -> String {
    print!("> ");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read input");;

    return buffer.trim().to_string();
}*/