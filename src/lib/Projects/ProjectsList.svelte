<script>
    import { invoke } from "@tauri-apps/api";
    import { entire_dir } from "../../stores/gen_dir";
    $: projs = [];
    $: entire_dir.subscribe((dir)=>projs=dir); 
    let start_from = (num, word) => {
        let new_path = ""; 
        while (num<=word.length){
            if (word[num]!==undefined){
                new_path = new_path + word[num]; 
                num++; 
            } else {
                num++; 
            }
        }
        return new_path; 
    };
    $: (()=>{
        let new_vector=[]; 
        let counter = 0; 
        while (counter<projs.length){
            let letter_counter = 1; 
            while (letter_counter<projs[counter]["title"].length){
                if (projs[counter]["title"][letter_counter]=="."){
                    projs[counter]["title"]=""; 
                    break; 
                }
                ++letter_counter; 
            }
            ++counter; 
        }
        let new_counter = 0; 
        while (new_counter<projs.length){
            if (projs[new_counter]["title"]!=""){
                new_vector.push(projs[new_counter]); 
            }
            ++new_counter; 
        }
        projs = new_vector; 
    })()
    let open_code = async (app_name) => {
        console.log("clicked");
        await invoke('open_vscode', {folderName: app_name}); 
    }; 
    let delete_project = async (app_name) => {
        let new_vec = []; 
        let counter = 0; 
        await invoke('erase_project', {currentDir: app_name});
        while (counter<projs.length){
            if (projs[counter]["title"] != app_name){
                new_vec.push(projs[counter]); 
            }
            ++counter; 
        }
        projs = new_vec; 
    }; 
</script>

<main class:showBorder={projs.length>3}>
    {#each projs as project}
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <section 
            on:mouseenter={()=>project["showBtns"]=true} 
            on:mouseleave={()=>project["showBtns"]=false} 
            on:click={()=>open_code(project["title"])}>
            <span>
                {start_from(2,project["title"])}
            </span>
            {#if project["showBtns"]}
                <button on:click|stopPropagation={()=>delete_project(project["title"])}>
                Delete
                </button>
            {/if}
        </section>
    {/each}
</main>

<style>
    button{
        width:85px;
        height:35px;
        font-size:19px;
        font-weight:bolder;
        background-color: #2b2a2a;
        border-radius:7px;
        margin-top:auto;
        margin-bottom:15px;
    }
    button:hover{
        border-width:3px;
        font-size:20px;
        background-image: linear-gradient(to right, #2b2a2a, black);
    }
    main{
        overflow-x: auto;
        height:250px;
        width:600px;
        border-radius: 7px;
        padding:5px;
        display: flex;
        align-items: center;
        justify-items: center;
        border:2px solid darkslategray;
    }
    .showBorder{
        border:2px solid whitesmoke;
    }
    section{
        display:flex;
        flex-direction: column;
        justify-content: start;
        align-items: center;
        padding:2px 3px;
        height:150px;
        width:140px;
        font-size: 24px;
        background-color: rgb(50, 51, 53);
        border-radius:7px;
        box-shadow: -1rem 0 3rem #000;
        margin-right:-30px;
        transition: .4s;
    }
    section:hover{
      background-image:linear-gradient(to bottom left, rgb(36, 34, 34), rgb(94, 90, 90));
        color:turquoise;
        border:1px solid yellow;
        margin-right:-5px;
    }
</style>
