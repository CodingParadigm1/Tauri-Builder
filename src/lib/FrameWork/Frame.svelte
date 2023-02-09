<script>
    export let icon="";
    export let description="";  
</script>
<!-- svelte-ignore a11y-click-events-have-key-events -->
<main>
    <img src={icon} alt="#" on:click>
    <input type="text" name="app-title" bind:value={description} placeholder="Enter the apps name.">
</main>

<style>
    main{
        display: flex;
        flex-direction:row;
        justify-content: space-between;
        align-items: center;
        width:550px;
        margin-bottom: 15px;
        background-color:rgb(139, 136, 136);
        padding:5px 7px;
        border-radius:7px;
        border:1px solid grey;
        transition:.5s;
    }
    main:hover{
        background-image:linear-gradient(to left top, black, rgb(139, 136, 136));
        border:1px gold solid;
    }
    img{
        height:65px;
        width:65px;
    }
    input{
        padding: 2px 5px;
        border-radius:7px;
        border:none;
        font-size:18px;
        margin-right: 20px;
        transition:.5s;
        border:yellow solid 1px;
    }
    input:focus{
        background-image: linear-gradient(to bottom left, rgb(139, 136, 136), black);
        border:none;font-size:19px;
    }
    input:hover{
        background-image: linear-gradient(to bottom left, rgb(139, 136, 136), black);
        font-size:19px;
        border-width: 2px;
    }
</style>