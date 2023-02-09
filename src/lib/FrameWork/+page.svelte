<script>
    import TitlePage from "../MainPage/TitlePage.svelte";
    import Frame from "./Frame.svelte";
    import { entire_dir } from "../../stores/gen_dir";
    import { invoke } from "@tauri-apps/api";
    let icons = ["/svelte.svg","/next.svg","/solid.svg","/react.svg"]; 
    $: titles = [
        "",
        "",
        "",
        ""
    ];
    let index = [0,1,2,3]; 
    let frameworks_tauri = ["svelte", "nextjs", "solid", "react"]; 
    let new_tuple = []; 
    let zip = (list1, list2, list3) => {
        let count = 0; 
        let new_list = []; 
        while (count<4){
            new_tuple = [list1[count], list2[count], list3[count]]; 
            console.log(new_tuple);
            new_list.push(new_tuple); 
            count++; 
        }
        return new_list; 
    };
    let clicked = async(fw, app_name)=>{
        let current_framework = ""+fw; 
        await invoke('generate_tauri_app', {framework:current_framework, appname: app_name});
    }
    $: data = zip(icons, frameworks_tauri, index); 
    let description="Here are the available frameworks for automated creation. Click the icons below to create a new project. Good luck!";
    let projects = []; 
    entire_dir.subscribe((dir)=>projects=dir); 
</script>

<main>
    <TitlePage title="Frameworks" desc={description}/>
    {#each data as d}
        <Frame icon={d[0]} bind:description={titles[d[2]]} on:click={()=>clicked(d[1], titles[d[2]])}/>
    {/each}
</main>

<style>
    main{
        background-color: rgb(66, 61, 61);
        width:675px;
        display:flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        margin-top:-65px;
        }
</style>