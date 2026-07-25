declare module "vite-plugin-top-level-await" {
	import type { Plugin } from "vite";

	const topLevelAwait: () => Plugin;
	export default topLevelAwait;
}
