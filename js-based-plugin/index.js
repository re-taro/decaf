import { createUnplugin } from "unplugin";
import { experimental_build as decaf_build, just_imports } from "decaf/initialised";
import { readFileSync } from "node:fs";

/// <reference path="types.d.ts"/>

function emitDiagnostics(on, diagnostics, plugin) {
	const lineSplits = [0];
	for (const line of on.split("\n")) {
		lineSplits.push(lineSplits.at(-1) + line.length + 1)
	}
	for (const diagnostic of diagnostics) {
		const line = lineSplits.findIndex(count => count > diagnostic.position.start);
		const column = diagnostic.position.start - lineSplits[line - 1];
		plugin.warn(diagnostic.reason, { line, column })
	}
}

/** @param {import("./types").DecafUnpluginOptions} options  */
function plugin(options = {}) {
	let allJsTsFiles = options.allJsTsFiles ?? false;
	const build = options.customBuild ?? decaf_build;

	const extensions = ["ts", "tsx", "js", "jsx"];

	const name = "decaf";
	const esbuild = {
		name,
		setup(esbuild) {
			esbuild.onLoad({ filter: /\.ts(x?)$/ }, async ({ path }) => {
				const code = readFileSync(path, 'utf8');
				try {
					const imports = just_imports(code);
					if (typeof imports === "string") {
						return { contents: imports };
					} else {
						throw Error("Issue parsing");
					}
				} catch (e) {
					return { errors: [] };
				}
			});
		},
	};
	return {
		name,
		vite: {
			enforce: 'pre',
			configResolved(config) {
				config.optimizeDeps.esbuildOptions.plugins = [esbuild];
			},
		},
		transformInclude(id) {
			const extension = id.split(".");
			const jsTsLikeExtension = extensions.includes(extension.at(-1));
			if (allJsTsFiles) {
				return jsTsLikeExtension;
			} else {
				return jsTsLikeExtension && extension.at(-2) == "decaf";
			}
		},
		transform(code, path) {
			/** Passed to Decaf's builder so it can import more */
			function readFile(pathDecafWantsToRead) {
				if (pathDecafWantsToRead !== path) {
					console.error(`tried to import '${pathDecafWantsToRead}' which is currently unsupported by the plugin`)
					return null;
				} else {
					return code;
				}
			}

			const output = build(path, readFile, false);
			emitDiagnostics(code, output.diagnostics, this)
			if (output.artifacts.length) {
				return {
					code: output.artifacts[0]
				}
			} else {
				this.warn("decaf had errors and did not transform");
				return code;
			}
		},
	};
}

export default createUnplugin(plugin);
