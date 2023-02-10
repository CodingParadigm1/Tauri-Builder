pub mod manage{
    use tauri; 
    use std::fs; 
    use std::path::{Path, PathBuf};
    use std::process::Command; 
    use std::sync::Arc; 
    use std::thread; 
        #[tauri::command]
        pub fn get_dir<'s>(cur_path: String)->Vec<PathBuf>{
            let path: &Path = Path::new(&cur_path); 
            let mut extracted_files: Vec<PathBuf> = Vec::new(); 
            for file in fs::read_dir(path).expect("Can't read directory"){
                    extracted_files.push(file.unwrap().path());                  
                }
                extracted_files
            }
        #[tauri::command]
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
        #[tauri::command]
        pub fn open_vscode(folder_name: &str){
            Command::new("cmd").args(["/c", "code", folder_name]).output().expect("Unable to run vscode."); 
        }
}
