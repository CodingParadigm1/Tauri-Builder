<script>
    export let web_page="";
    export let used_icon=""; 
    export let description="This is a default description of a framework."; 
</script>

<main>
    <article>
        <img src={used_icon} alt="#">
        <p>
            {description}
        </p>
    </article>
    <embed src={web_page} data-sveltekit-preload-code="eager">
</main>

<style>
    img{
        height:55px;
        width:55px;
    }
    main{
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        margin-top:5px;
        width:675px;
    }
    article{
        display:flex;
        flex-direction: row;
        justify-content: space-around;
        margin-bottom:15px;
        width:380px;
        background-color:rgb(122, 119, 119);
        padding:10px 8px;
        border-radius:7px;
        border:solid grey 1px;
    }
    article:hover{
        border:gold 1px solid;
    }
    embed{
        width:500px;
        height:270px;
        border-radius:7px;
    }
    p{
        margin-top:5px;
        width:245px;
        color:turquoise;
        font-size:16px;
    }
</style>
