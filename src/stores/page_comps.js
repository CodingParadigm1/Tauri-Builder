import { writable } from "svelte/store";
import FrameWork from "../lib/FrameWork/+page.svelte"; 
import MainPage from '../lib/MainPage/+page.svelte'; 
import Projects from '../lib/Projects/+page.svelte'; 
import License from '../lib/License/+page.svelte'; 

export let page_component = writable([
    {"page":MainPage,"title":"Home", "index":0}, 
    {"page":FrameWork,"title":"frameworks", "index":1}, 
    {"page":Projects, "title":"projects","index":2}, 
    {"page": License, "title":"license", "index": 3}
]); 
