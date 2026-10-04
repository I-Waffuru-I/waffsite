
import { getBlogPost, getBlogList, parseMarkdownToHtml } from "./util.js";

document.getElementById("current_blog").style.display = 'none'
let list = await getBlogList()
console.log("after getting list:")
console.log(list)
renderBlogList(list)

document.getElementById("bloglist").addEventListener("click", async function(e) {
	let title = e.target.innerText
	const res = list.find((element) => element['title'] == title)
	console.log(`found ${res}`)
	if (res != undefined) {
		let post = await getBlogPost(res)
		const el = document.getElementById("current_blog")
		el.innerHTML = parseMarkdownToHtml(post, `blog/data/${res['dir']}`)
		el.style.display = 'block'
	}
});


function renderBlogList(ls){
	if (ls != undefined && ls != null) {
		let element = document.getElementById("bloglist")
		if (ls.length > 0){
			let s = "<ul>"
			for (let val of ls) {
				s += "<li>"+val['title']+"</li>"
			}
			s += "</ul>"
			element.innerHTML = s
		} else {
			element.innerHTML = "<h4>There are no blogs yet...</h4>"
		}
	}
}

