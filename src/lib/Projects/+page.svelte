<script>
    import TitlePage from "../MainPage/TitlePage.svelte";
    import ProjectsList from "./ProjectsList.svelte";
    import {invoke} from "@tauri-apps/api"; 
    import { entire_dir } from "../../stores/gen_dir";
    let description = "Below are your created projects. If there are no projects, this app has created an example of one for you."

    let projects = []; 
    (async ()=>{
            projects = await invoke('get_dir', {curPath: "./"})
    })();
    $: entire_dir.set(projects); 
</script>

<main>
    <TitlePage title="Projects" desc={description}/>
    <ProjectsList/>
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