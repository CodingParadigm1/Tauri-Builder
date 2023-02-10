<script>
	import { page_component } from './../../stores/page_comps.js';
	import { page_number } from '../../stores/page_num.js';
    let pages; 
    page_component.subscribe((page)=>{
        pages = page; 
    }); 
    let onClick = (page_num) => {
        page_number.set(page_num); 
    };
    onClick(0); 
</script>
<article>
    <ul>
        {#each pages as page}
             <!-- svelte-ignore a11y-click-events-have-key-events -->
             <li on:click={()=>onClick(page["index"])}>
                {page["title"]}
             </li>
        {/each}
    </ul>
</article>
<style>
    *{
        font-size: 18px;
        color:turquoise;
        background-color: rgb(20, 15, 15);
    }
    article{
        width:195px;
        height:800px;
        margin-bottom:auto;
        margin-top:-20px;
    }
    ul{
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        margin-top:25px;
 
        width:100%;
    }
    li{
        list-style: none;
        margin-top:15px;
        width:98%;
        height:30px;
        text-align: center;
    }
    li:nth-child(1){
        margin-top:25px;
    }
    li:hover{
        background-color: rgb(2, 4, 17);
        border:1px yellow solid;
        border-right:none;
        border-left:none;
        padding-top: 3px;
    }
</style>
