

export const CACHE_LIST_KEY = "cachelistkey";

export class CachedList {
	value;
	expire_time;

	constructor(){}

	static fromValue(value) {
		let x = Object.assign(new CachedList())
		x.value = value;
		let date = Date.now();
		x.expire_time = date + 50000;
		return x
	}
	toStr() {
		return JSON.stringify(this)
	}
	static fromStr(str) {
		return Object.assign(new CachedList(), JSON.parse(str));
	}
}

