use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

const VERSION_FILE: &str = "./.vcs/version";

pub fn init() {
    match fs::create_dir("./.vcs") {
        Ok(_) => println!("Repositório inicializado!"),
        Err(_) => println!("Erro ao inicializar repositório!"),
    }
}

pub fn commit(description: &str) {
    let version = match fs::read_to_string(VERSION_FILE) {
        Ok(value) => value.parse::<i32>().expect("Erro ao converter versão") + 1,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => 1,
            _ => panic!("Erro ao converter versão"),
        },
    };

    let version_directory = format!("./.vcs/{version}");
    fs::create_dir(&version_directory).expect("Erro ao criar diretório de versão");
    fs::write(format!("{version_directory}/README"), format!("{description}\n")).unwrap();
    persist_directory(Path::new("."), Path::new(&format!("./.vcs/{version}")));
    fs::write("./.vcs/version", format!("{version}")).expect("Falha ao atualizar versão");
}

fn persist_directory(from: &Path, to: &Path) {
    let dir = fs::read_dir(from).expect("Erro ao ler diretório");
    for item in dir {
        let item = item.expect("Erro ao ler caminho").path();

        let file_name = item.file_name().expect("Erro");
        if file_name == ".vcs" || file_name == "README" {
            continue;
        }

        if item.is_dir() {
            let mut dir_to = PathBuf::from(to);
            dir_to.push(item.file_name().expect("Erro"));
            fs::create_dir(&dir_to).expect("Erro");
            persist_directory(&item, &dir_to)
        } else {
            let copy_to: PathBuf = [to, Path::new(item.file_name().expect("Erro"))]
                .iter()
                .collect();
            println!("Persistindo {}", copy_to.to_str().expect("Erro"));
            fs::copy(item, copy_to).expect("Erro");
        }
    }
}

pub fn restore(version: u32) {
    clear_directory(".");
    persist_directory(Path::new(&format!("./.vcs/{version}")), Path::new("."));
}

fn clear_directory(directory: &str) {
    let root_dir = fs::read_dir(directory).unwrap();
    for item in root_dir {
        let item_path = item.unwrap().path();
        let file_name = item_path.file_name().unwrap();

        if file_name == ".vcs" {
            continue;
        }

        if item_path.is_dir() {
            fs::remove_dir_all(item_path).unwrap();
        } else {
            fs::remove_file(item_path).unwrap();
        }
    }
}
