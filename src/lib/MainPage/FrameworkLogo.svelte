<script>
    export let icon_img = ""; 
</script>

<span>
    <img src={icon_img} alt="#" class="icon" on:click>
</span>

<style>
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
</style>