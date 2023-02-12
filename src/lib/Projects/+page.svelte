<script>
	import TitlePage from './../MainPage/TitlePage.svelte';
    import ProjectsList from "./ProjectsList.svelte";
    import {invoke} from "@tauri-apps/api"; 
    import { entire_dir } from "../../stores/gen_dir";
    let description = "Below are your created projects. If there are no projects, this app has created an example of one for you."
    let hint = "Tip: click the card to open vscode. Click 'delete' to delete the project."; 
    let projects = []; 
    
    (async ()=>{
            projects = await invoke('get_dir', {curPath: "./"})
    })();
    let indexed_projs = []; 
    $: (()=>{
        let counter = 0; 
        while (counter<projects.length){
            let indexed_project = {"title": projects[counter], "index": counter, "showBtn": false}; 
            indexed_projs = [indexed_project, ...indexed_projs]; 
            ++counter; 
    }})(); 
    $: entire_dir.set(indexed_projs); 
</script>

<main>
    <TitlePage title="Projects" desc={description}/>
    <ProjectsList/>
    <TitlePage title="" desc={hint}/>
</main>

<style>
    main{
        background-color: rgb(66, 61, 61);
        display:flex;
        flex-direction: column;
        align-items: center;
        width:675px;
    }
</style>
