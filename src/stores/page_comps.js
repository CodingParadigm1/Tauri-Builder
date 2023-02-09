import { writable } from "svelte/store";
import FrameWork from "../lib/FrameWork/+page.svelte"; 
import MainPage from '../lib/MainPage/+page.svelte'; 
import Projects from '../lib/Projects/+page.svelte'; 


export let page_component = writable([
    {"page":MainPage,"title":"main menu", "index":0}, 
    {"page":FrameWork,"title":"frame works", "index":1}, 
    {"page":Projects, "title":"projects","index":2}
]); 