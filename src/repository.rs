use std::{
    fs,
    io::{self, ErrorKind},
    path::Path,
};

const VCS_DIR: &str = ".vcs";
const VERSION_FILE: &str = "version";

pub fn init() {
    match fs::create_dir(VCS_DIR) {
        Ok(_) => println!("Repositório inicializado!"),
        Err(_) => println!("Erro ao inicializar repositório!"),
    }
}

pub fn commit(description: &str) {
    let version = get_latest_version().unwrap_or(0) + 1;
    let version_directory = format!("{VCS_DIR}/{version}");
    fs::create_dir(&version_directory).expect("Erro ao criar diretório de versão");
    fs::write(
        format!("{version_directory}/README"),
        format!("{description}\n"),
    )
    .unwrap();
    persist_directory(Path::new("."), Path::new(&version_directory)).unwrap();
    fs::write(format!("{VCS_DIR}/{VERSION_FILE}"), format!("{version}"))
        .expect("Falha ao atualizar versão");
}

fn get_latest_version() -> Option<u32> {
    let version_file_path = format!("{VCS_DIR}/{VERSION_FILE}");
    let incorrect_version_format_message =
        format!("Formato de versão em {version_file_path} incorreto");
    match fs::read_to_string(&version_file_path) {
        Ok(value) => Some(
            value
                .parse::<u32>()
                .expect(&incorrect_version_format_message),
        ),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => None,
            _ => panic!("Erro ao ler arquivo de versão"),
        },
    }
}

fn persist_directory(source: &Path, dest: &Path) -> Result<(), io::Error> {
    for entry in fs::read_dir(source)? {
        let entry = entry?;

        if entry.file_name() == VCS_DIR || entry.file_name() == "README" {
            continue;
        }

        if entry.file_type()?.is_dir() {
            let to_dir = dest.join(entry.file_name());
            fs::create_dir(&to_dir)?;
            persist_directory(&entry.path(), &to_dir)?;
        } else {
            fs::copy(&entry.path(), dest.join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub fn restore(version: u32) {
    clear_directory(".");
    persist_directory(Path::new(&format!("{VCS_DIR}/{version}")), Path::new(".")).unwrap();
}

fn clear_directory(directory: &str) {
    let root_dir = fs::read_dir(directory).unwrap();
    for item in root_dir {
        let item_path = item.unwrap().path();
        let file_name = item_path.file_name().unwrap();

        if file_name == VCS_DIR {
            continue;
        }

        if item_path.is_dir() {
            fs::remove_dir_all(item_path).unwrap();
        } else {
            fs::remove_file(item_path).unwrap();
        }
    }
}
