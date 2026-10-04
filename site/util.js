
// yanked from and built upon
// https://randyperkins2k.medium.com/writing-a-simple-markdown-parser-using-javascript-1f2e9449a558
export function parseMarkdownToHtml(text, prefix) {
	const toHTML = text
		.replace(/^$/gim, '<br /><br />')
		.replace(/\\$/gim, '<br />')
		.replace(/\!\[\[(.*?)\]\]/gim, `<img src="${prefix}/$1">`)
		.replace(/^### (.*?$)/gim, '<h3>$1</h3>') 
		.replace(/^## (.*?$)/gim, '<h2>$1</h2>') 
		.replace(/^# (.*?$)/gim, '<h1>$1</h1>') 
	// (?:\`{3}|\`{1})(.*?)(?:\`{3}|\`{1})
	   .replace(/```(.*?)```/gims, "<pre><code class=\"code\">$1</code></pre>")
		.replace(/`(.*?)`/gim, "<code class=\"code\">$1</code>")
		.replace(/\*\*(.*?)\*\*/gim, '<b>$1</b>')
		.replace(/\*(.*?)\*/gim, '<i>$1</i>');

	const codedHtml = toHTML.replace(/<code class="code">(.*?)<\/code>/gs, (_, val) => {
		const newval = val
			.replace(/\</g, "&lt;")
			.replace(/\>/g, "&gt;");
		return `<code class="code">${newval}</code>`
	})
	return codedHtml.trim();
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

