
// yanked from and built upon
// https://randyperkins2k.medium.com/writing-a-simple-markdown-parser-using-javascript-1f2e9449a558
export function parseMarkdownToHtml(text) {
	const toHTML = text
		.replace(/^### (.*$)/gim, '<h3>$1</h3>') 
		.replace(/^## (.*$)/gim, '<h2>$1</h2>') 
		.replace(/^# (.*$)/gim, '<h1>$1</h1>') 
		.replace(/\*\*(.*)\*\*/gim, '<b>$1</b>') 
		.replace(/\*(.*)\*/gim, '<i>$1</i>'); 
	return toHTML.trim();
}

export async function getBlogPost(blog) {
	const request = new Request(`/blog/data/${blog['dir']}/${blog['file']}`)
	return await fetch(request)
		.then(async (resp) => {
			if (resp.status == 200) {
				return await resp.text()
			} 
		})
		.catch((e) => {console.log(e)})
}

export async function getBlogList(){
	return await fetchList()
}

async function fetchList(){
	const request = new Request("/blog/list")
	return await fetch(request)
		.then(async (resp) => {
			if (resp.status == 200) {
				/*
				let list = CachedList.fromValue(await resp.text());
				window.localStorage.setItem(CACHE_LIST_KEY, list.toStr());
				console.log("success getting list"+list)
				return list
				*/
				return resp.json()
			} 
		})
		.catch((e) => {console.log(e)})
}

