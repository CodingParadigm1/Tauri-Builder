pub mod manage{
    use tauri; 
    use std::fs::{self, remove_dir_all, remove_file}; 
    use std::path::{Path, PathBuf};
    use std::process::Command; 
    use std::sync::Arc; 
    use std::thread;  
    use std::fmt::Debug; 
    use std::io; 
    use std::env; 
        #[tauri::command(async)]
        pub fn scan_files(titles: Vec<&str>){
            let mut counter: usize = 0; 
            while counter<titles.len(){
                if titles[counter]=="./tauri-svelte"{
                    return
                }
                counter = counter + 1; 
            }
            generate_tauri_app("initial".to_string(), "".to_string()); 
        }
        #[tauri::command(async)]
        pub fn get_dir<'s>(cur_path: String)->Vec<PathBuf>{
            let path: &Path = Path::new(&cur_path); 
            let mut extracted_files: Vec<PathBuf> = Vec::new(); 
            for file in fs::read_dir(path).expect("Can't read directory"){
                    extracted_files.push(file.unwrap().path());                  
                }
                extracted_files
            }
        #[tauri::command(async)]
        pub fn generate_tauri_app(framework: String, appname: String){ 
            let no_title = format!("tauri-{}", &framework.as_str()); 
            let mut gen_app_name: Arc<String> = Arc::new(appname.clone()); 
            println!("{}", appname.clone().len()); 
            if appname.len()==0{  
                gen_app_name = Arc::new(no_title); 
            } 
            match framework.as_str(){
                "initial"=>{
                    let cmd_command = thread::spawn(||{
                        Command::new("pnpm").args(["create", "tauri-app", "tauri-svelte", "--manager", "pnpm", "--template", "svelte-kit", "--yes"]).output().expect("unable ot create tauri app");  
                    });
                    cmd_command.join().unwrap(); 
                },
                "svelte"=>{
                    let cmd_command = thread::spawn(|| {
                        Command::new("pnpm").args(["create", "tauri-app", &Arc::try_unwrap(gen_app_name).unwrap().as_str(), "--manager", "pnpm", "--template", "svelte-kit", "--yes"]).output().expect("unable ot create tauri app");
                    }); 
                    cmd_command.join().unwrap();  
                }, 
                "nextjs"=>{
                    let cmd_command = thread::spawn(|| { 
                    Command::new("pnpm").args(["create", "tauri-app", &Arc::try_unwrap(gen_app_name).unwrap().as_str(), "--manager", "pnpm", "--template", "next", "--yes"]).output().expect("unable ot create tauri app"); 
                    }); 
                cmd_command.join().unwrap();
                }, 
                "solid"=>{
                    let cmd_command = thread::spawn(|| {
                    Command::new("pnpm").args(["create", "tauri-app",&Arc::try_unwrap(gen_app_name).unwrap().as_str(), "--manager", "pnpm", "--template", "solid", "--yes"]).output().expect("unable ot create tauri app"); 
                    }); 
                    cmd_command.join().unwrap();
                }, 
                "react"=>{
                    let cmd_command = thread::spawn(|| {
                    Command::new("pnpm").args(["create", "tauri-app", &Arc::try_unwrap(gen_app_name).unwrap().as_str(), "--manager", "pnpm", "--template", "react", "--yes"]).output().expect("unable ot create tauri app"); 
                    }); 
                    cmd_command.join().unwrap();
                }, 
                _=>()
            }
        }
        #[tauri::command(async)]
        pub fn open_vscode(folder_name: String){
            let folder = Arc::new(folder_name); 
            let cmd_command = thread::spawn(|| {
             Command::new("cmd").args(["/c", "code", &Arc::try_unwrap(folder).unwrap()]).output().expect("Unable to run vscode."); 
            }); 
            cmd_command.join().unwrap();
        }
        #[derive(Debug)]
        enum ProgError {
            NoFile,
            NotUtf8,
            Io(io::Error),
        }
        
        impl From<io::Error> for ProgError {
            fn from(err: io::Error) -> ProgError {
                ProgError::Io(err)
            }
        }
        
        fn current_app_name() -> Result<String, ProgError> {
            let program_name = env::current_exe()?
                .file_name().ok_or(ProgError::NoFile)?
                .to_str().ok_or(ProgError::NotUtf8)?
                .to_owned();
            Ok(program_name)
        }
        fn no_folders(current_dir: String)->io::Result<bool>{
            let string_path = format!("./{current_dir}"); 
            let new_path = Path::new(string_path.as_str()); 
            for folders in fs::read_dir(new_path)?{ 
                if folders.is_ok(){
                    if folders?.path().is_dir(){
                        return Ok(false); 
                    }
                }
            }
            Ok(true) 
        }
        #[tauri::command(async)]
        pub fn erase_project(current_dir: String){ 
            let string_path = format!("./{current_dir}"); 
            let new_path = Path::new(string_path.as_str());
            let app_path: String = format!("../test_files\\{}", current_app_name().unwrap().as_str()); 
            let mut deleted_dirs:Vec<PathBuf> = Vec::new(); 
            let mut deleted_files: Vec<PathBuf> = Vec::new(); 
            let mut counter = 0; 
            while counter<2{
                for folder in fs::read_dir(new_path).unwrap(){  
                    if folder.is_ok(){
                        let unwrapped_folder = folder.unwrap(); 
                        let folders_path = unwrapped_folder.path(); 
                        let path_not_exe = folders_path.display().to_string().as_str() != app_path.as_str();
                        if no_folders(current_dir.clone()).unwrap()==false { 
                            if folders_path.is_dir() && path_not_exe{ 
                                if remove_dir_all(folders_path.clone()).is_ok(){
                                    deleted_dirs.push(folders_path); 
                                } 
                            } else {
                                continue; 
                            }
                        } else {
                            if path_not_exe  {
                                if remove_file(folders_path.clone()).is_ok(){
                                    deleted_files.push(folders_path);
                                } 
                            }
                        }
                    }
                }
                counter = counter + 1; 
            }
            let num_files = deleted_files.len();
            let num_dir_deleted = deleted_dirs.len();  
            println!("{num_files} files deleted"); 
            println!("{num_dir_deleted} directories deleted"); 
            remove_dir_all(current_dir).unwrap();  
        }
}
