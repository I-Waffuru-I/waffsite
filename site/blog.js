
import { getBlogPost, getBlogList, parseMarkdownToHtml } from "./util.js";

let list = await getBlogList()
console.log("after getting list:")
console.log(list)

if (list != undefined && list != null) {
	let element = document.getElementById("bloglist")
	let s = "<ul>"
	for (let val of list.value.split(";")) {
		s += "<li>"+val+"</li>"
	}
	s += "</ul>"

	element.innerHTML = s
}

document.getElementById("bloglist").addEventListener("click", async function(e) {
	let post = await getBlogPost(e.target.innerText)
	document.getElementById("current_blog").innerHTML = parseMarkdownToHtml(post)
});



