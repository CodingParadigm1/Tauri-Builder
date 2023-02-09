<script>
    export let title = "Tauri Builder"; 
    export let desc = "Tauri builder is a beginner friendly alternative to creating tauri apps for popular Frameworks!"; 
</script>

<main>
    <h1>
        {title}
    </h1>
    <article>
        <p>
            {desc}
        </p>
    </article>
</main>

<style>
    p{
        text-align: center;   
        color:turquoise;  
        font-size:17px; 
    }
    article{
        margin-top:5px;
        width:450px;
        background-color:rgb(122, 119, 119);
        padding:10px 8px;
        border-radius:7px;
        transition: .3s;
        border:solid grey 1px;
    }
    article:hover{
        border:yellow 1px solid;
    }
    h1{
        margin:20px auto 15px auto;
    }
    main{
        background-color: rgb(66, 61, 61);
        display:flex;
        flex-direction: column;
        align-items: center;
        justify-content: start;
        margin-bottom:50px;
        width:600px;
    }
</style>