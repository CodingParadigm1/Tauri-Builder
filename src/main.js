import "./style.css";
import App from "./App.svelte";
import "@fontsource/rubik"; 
import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api";
const app = new App({
  target: document.getElementById("app"),
});
let projects = [];

(async ()=>{
  projects = await invoke('get_dir', {curPath: "./"})
})();
(async () => {
  let counter = 0; 
  while (counter<projects.length){
      if (projects[counter] === "./tauri-svelte"){
          return; 
      }
      ++counter; 
  }
  await invoke('generate_tauri_app', {framework:"initial", appname: ""});
})(); 
export default app;
