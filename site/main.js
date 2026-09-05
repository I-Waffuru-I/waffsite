
import { CachedList, CACHE_LIST_KEY } from "./types.js";

console.log("before")
let needFetch = false;

let storageCachedList = window.localStorage.getItem(CACHE_LIST_KEY);
if (storageCachedList == null || storageCachedList == "") {
	needFetch == true;
}

let cachedList = new CachedList()
if (!needFetch) {
	cachedList  = CachedList.fromStr(storageCachedList)

}
if (cachedList.expire_time <= Date.now()){
	needFetch = true;
}

if (needFetch) {
	console.log("Fetching list of blogs")
	fetchList()
}

function fetchList(){
	const request = new Request("http://localhost:8000/blog/list")
	fetch(request)
		.then(async (resp) => {
			if (resp.status == 200) {
				cachedList = CachedList.fromValue(await resp.text());
				window.localStorage.setItem(CACHE_LIST_KEY, cachedList.toStr());
				console.log("success")
			} 
			else {
				console.log("failure")
				throw new Error(`error: `+resp.status);
			}
		})
		.catch((e) => {console.log(e)})
	
}
