import "./style.css";
import App from "./App.svelte";
import "@fontsource/rubik"; 
import { invoke } from "@tauri-apps/api";

const app = new App({
  target: document.getElementById("app"),
});
export default app;

let projects = [];
(async ()=>{
  projects = await invoke('get_dir', {curPath: "./"}); 
  await invoke('scan_files', {titles: projects}); 
})(); 
