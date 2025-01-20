import Shiki from "@shikijs/markdown-it";

const shiki = await Shiki({
	themes: {
		dark: "vitesse-dark",
		light: "vitesse-light",
	},
});

export default ({ marp }) => marp.use(shiki);
