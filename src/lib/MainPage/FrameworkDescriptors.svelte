<script>
    import FrameDescripts from '../FrameWorkEmbed/+page.svelte'; 
    import FrameworkLogo from './FrameworkLogo.svelte';
    let icons = [{"title":"/svelte.svg", "index":0},{"title":"/next.svg", "index":1},{"title":"/tauri.svg", "index":2},{"title":"/solid.svg", "index":3},{"title":"/react.svg", "index":4}]; 
    let descriptions = [
        "Svelte compiles your javascript code into enhanced js.",
        "If you need bleeding edge, this framework might be for you.",
        "Tauri: built on rust is the glue to apps like the one you're using.",
        "Solid is the answer to compiled performance, when compared to react.",
        "Taking advantage of JSX, React takes a battle tested approach to UI dev."]; 
    let web_sites = ["https://kit.svelte.dev/", "https://nextjs.org/","https://tauri.app/","https://www.solidjs.com/", "https://reactjs.org/"]; 
    $: counter = 0; 
    $: framework_counter = 0; 
    let change_framework = (num) => {
        counter = num; 
    };
</script>

<main>
    <article id="frameworks">
        {#each icons as icon}
             <FrameworkLogo icon_img={icon["title"]} on:click={()=>change_framework(icon["index"])}/>
        {/each}
    </article>
</main>
<FrameDescripts used_icon={icons[counter]["title"]} description={descriptions[counter]} web_page={web_sites[counter]}/>
<style>
    main{
        margin-top:-20px;
        margin-bottom: 15px;
    }
    #frameworks{
        display: flex;
        flex-direction: row;
        justify-content: space-around;
        align-items: center;
        width:470px;
        height: 87px;
    }
</style>
