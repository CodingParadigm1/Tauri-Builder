<script>
    import { invoke } from "@tauri-apps/api";
    import { entire_dir } from "../../stores/gen_dir";
    let projs = [];
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
            while (letter_counter<projs[counter].length){
                if (projs[counter][letter_counter]=="."){
                    projs[counter]=""; 
                    break; 
                }
                ++letter_counter; 
            }
            ++counter; 
        }
        let new_counter = 0; 
        while (new_counter<projs.length){
            if (projs[new_counter]!=""){
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
</script>
<main class:showBorder={projs.length>3}>
    {#each projs as project}
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <section on:click={()=>open_code(project)}>
            <span>
                {start_from(2,project)}
            </span>
        </section>
    {/each}
</main>

<style>
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
        padding:2px 3px;
        height:150px;
        width:200px;
        font-size: 24px;
        min-width:140px;
        background-color: rgb(50, 51, 53);
        border-radius:7px;
        box-shadow: -1rem 0 3rem #000;
        margin-right:-30px;
        transition: .2s;
    }
    section:hover{
      background-image:linear-gradient(to bottom left, rgb(36, 34, 34), rgb(94, 90, 90));
        color:turquoise;
        border:1px solid yellow;
        margin-right:-5px;

    }
    
</style>
