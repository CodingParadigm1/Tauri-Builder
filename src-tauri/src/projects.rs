pub mod manage{
    use tauri; 
    use std::fs; 
    use std::path::{Path, PathBuf};
    use std::process::Command; 
    use std::cell::RefCell; 
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
            let no_title = format!("tauri-{}",&framework); 
            let gen_app_name: RefCell<&str> = RefCell::new(&appname); 
            println!("{:?}", gen_app_name.clone().into_inner()); 
            if appname.to_string().len()==0{  
                *gen_app_name.borrow_mut() = no_title.as_str(); 
            } 
            match framework.as_str(){
                "initial"=>{
                    Command::new("pnpm").args(["create", "tauri-app", "tauri-svelte", "--manager", "pnpm", "--template", "svelte-kit", "--yes"]).output().expect("unable ot create tauri app");  
                },
                "svelte"=>{
                    //Command::new("mkdir").arg("projects").output().expect("unable to make directory");
                    Command::new("pnpm").args(["create", "tauri-app", gen_app_name.take(), "--manager", "pnpm", "--template", "svelte-kit", "--yes"]).output().expect("unable ot create tauri app"); 
                }, 
                "nextjs"=>{ 
                    Command::new("pnpm").args(["create", "tauri-app", gen_app_name.take(), "--manager", "pnpm", "--template", "next", "--yes"]).output().expect("unable ot create tauri app"); 
                }, 
                "solid"=>{
                    Command::new("pnpm").args(["create", "tauri-app",gen_app_name.take(), "--manager", "pnpm", "--template", "solid", "--yes"]).output().expect("unable ot create tauri app"); 
                }, 
                "react"=>{
                    Command::new("pnpm").args(["create", "tauri-app", gen_app_name.take(), "--manager", "pnpm", "--template", "react", "--yes"]).output().expect("unable ot create tauri app"); 
                }, 
                _=>()
            }
        }
        #[tauri::command]
        pub fn open_vscode(folder_name: &str){
            println!("{folder_name}"); 
            Command::new("cmd").args(["/c", "code", folder_name]).output().expect("Unable to run vscode."); 
        }
}