<script>
    import FrameDescripts from '../FrameWorkEmbed/+page.svelte'; 
    let icons = ["/svelte.svg","/next.svg","/tauri.svg","/solid.svg","/react.svg"]; 
    let descriptions = [
        "Svelte compiles your javascript code into enhanced js.",
        "If you need bleeding edge, this framework might be for you.",
        "Tauri: built on rust is the glue to apps like the one you're using.",
        "Solid is the answer to compiled performance, when compared to react.",
        "Taking advantage of JSX, React takes a battle tested approach to UI dev."]; 
    let web_sites = ["https://kit.svelte.dev/", "https://nextjs.org/","https://tauri.app/","https://www.solidjs.com/", "https://reactjs.org/"]; 
    $: counter = 0; 
    let change_framework = (num) => {
        counter = num; 
    };
</script>

<main>
    <article id="frameworks">
        <span>
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <img src="/svelte.svg" class="icon" alt="Svelte Logo" on:click={()=>change_framework(0)}/>
        </span>
        <span>
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <img src="/next.svg" alt="#"  class="icon" on:click={()=>change_framework(1)}>
        </span>
        <span>
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <img src="/tauri.svg" class="icon" alt="Svelte Logo" on:click={()=>change_framework(2)}/>
        </span>
        <span>
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <img src="/solid.svg" alt="#" class="icon"  on:click={()=>change_framework(3)}>
        </span>
        <span>
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <img src="/react.svg" alt="#" class="icon"  on:click={()=>change_framework(4)}>
        </span>
    </article>
</main>
<FrameDescripts used_icon={icons[counter]} description={descriptions[counter]} web_page={web_sites[counter]}/>
<style>
    main{
        margin-top:-20px;
        margin-bottom: 15px;
    }
    span{
        width:75px;
        height:75px;
        padding:2px;
        text-align: center;
        transition:.4s; 
        border-radius: 50%;
    }
    span:hover{
        background-image: radial-gradient( rgb(80, 77, 77), black);
        padding:5px;
    }
    #frameworks{
        display: flex;
        flex-direction: row;
        justify-content: space-around;
        align-items: center;
        width:470px;
        height: 87px;
    }
    .icon{
        width:65px;
        height:65px;
    }
    .icon:hover{
        width:70px;
        height:70px;
    }
    img{
        margin-top:2px;
    }













        /* span:nth-child(odd){
        width:75px;
        height:75px;
        padding:5px;
        text-align: center;
        transition:.4s;
        border-top-right-radius:50%;
        border-bottom-left-radius:50%;
    }
    span:hover:nth-child(odd){
        background-image: radial-gradient( rgb(80, 77, 77), black);
        border-top-left-radius:50%;
        border-bottom-right-radius:50%;
    }
    span:nth-child(even){
        width:75px;
        height:75px;
        padding:5px;
        text-align: center;
        transition:.4s;
        border-top-left-radius:50%;
        border-bottom-right-radius:50%;
    }
    span:hover:nth-child(even){
        background-image: radial-gradient( rgb(80, 77, 77), black);
        border-top-right-radius:50%;
        border-bottom-left-radius:50%;
    }
</style>
