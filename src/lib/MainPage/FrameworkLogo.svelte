<script>
    export let icon_img = ""; 
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<section on:click>
    <img src={icon_img} alt="#">
</section>

<style>
    img{
        width:65px;
        height:65px;
    }
    img:hover{
        width:70px;
        height:70px;
    }
    section{
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;
        width:75px;
        height:75px;
        padding:2px;
        transition:.4s; 
        border-radius: 50%;
        background-image: radial-gradient( rgb(99, 98, 98), black);
    }
    section:hover{
        padding:5px;
    }
</style>
