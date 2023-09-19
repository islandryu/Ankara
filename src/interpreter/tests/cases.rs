use std::fs;
use std::io::Write;
use std::path::Path;

#[cfg(test)]
mod tests {
    use std::{
        cell::RefCell,
        rc::{self, Rc},
    };

    use crate::{
        builtin::get_builtin_environment::get_builtin_environment,
        interpreter::evaluator::{EvalOption, Evaluator},
        lexer::Peekable,
        parser::parse,
        read_file::read_file,
    };

    use super::*;

    #[test]
    fn test_write_or_check_file() -> std::io::Result<()> {
        let all_case_file_path = get_all_case_file_path();

        for file_path in all_case_file_path {
            let code = read_file(&file_path)?;
            let mut env = get_builtin_environment();
            let rc_env = Rc::new(RefCell::new(env));
            let mut lexer = Peekable::new(&code);
            let program = parse(&mut lexer);
            match program
                .unwrap()
                .eval(rc_env.clone(), &mut EvalOption::new())
            {
                Ok(_) => {}
                Err(error) => {
                    println!("{:?}", error);
                    return Ok(());
                }
            }

            // last of path
            let file_name = file_path
                .split("/")
                .last()
                .unwrap()
                .split(".")
                .next()
                .unwrap();
            let text = (*rc_env.clone()).borrow_mut().to_string();
            let result = write_or_check_file(&file_name, &text)?;
            print!("{} \n", file_name);
            assert!(result);
            print!("\n")
        }

        Ok(())
    }
}

fn get_all_case_file_path() -> Vec<String> {
    let case_dir = fs::read_dir("./src/interpreter/tests/cases").unwrap();
    let mut file_paths = Vec::new();
    for entry in case_dir {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            file_paths.push(path.to_str().unwrap().to_string());
        }
    }
    file_paths
}

fn write_or_check_file(file_name: &str, content: &str) -> std::io::Result<bool> {
    let out_dir = "./src/interpreter/tests/outputs";
    let file_path = Path::new(out_dir).join(file_name).with_extension("txt");

    if file_path.exists() {
        let existing_content = fs::read_to_string(file_path)?;
        dbg!(&existing_content);
        dbg!(&content);
        Ok(existing_content == content)
    } else {
        let mut file = fs::File::create(file_path)?;
        match file.write_all(content.as_bytes()) {
            Ok(_) => {}
            Err(error) => {
                println!("{:?}", error);
                return Ok(false);
            }
        }
        Ok(true)
    }
}
