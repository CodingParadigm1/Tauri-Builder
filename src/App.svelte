<script>
	import { page_component } from './stores/page_comps.js';
  import {page_number} from './stores/page_num'; 
  import SideMenu from './lib/SideMenu/+page.svelte';
  $: counter = 0;
  let components; 
  page_number.subscribe((page_num)=>{
    counter=page_num; 
  })
  page_component.subscribe((db_component)=>{
    components = db_component; 
  })
  
</script>
<main>
  <SideMenu/>
  <svelte:component this={components[counter]["page"]}/>
</main>

<style>
  main{
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    width:850px;
    height:650px;

  }
</style>



