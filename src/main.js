import "./style.css";
import App from "./App.svelte";
import "@fontsource/rubik"; 
import { invoke } from "@tauri-apps/api";
let projects = [];

let render = async ()=>{
  const app = async () =>{ 
    return new App({
      target: document.getElementById("app"),
    })
  };
  const develop_data = async ()=>{
    projects = await invoke('get_dir', {curPath: "./"}); 
    await invoke('scan_files', {titles: projects}); 
  };
  return {
    projs: develop_data(), 
    app_data:  app()
  }
}; 

export default (await render()).app_data



 
