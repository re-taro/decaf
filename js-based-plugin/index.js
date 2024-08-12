import { createUnplugin } from "unplugin";
import { build } from "decaf/initialized";

function emitDiagnostics(on, diagnostics, plugin) {
  const lineSplits = [0];
  for (const line of on.split("\n")) {
    // TODO need buffer length not character count
    lineSplits.push(lineSplits.at(-1) + line.length + 1)
  }
  for (const diagnostic of diagnostics) {
    const line = lineSplits.findIndex(count => count >= diagnostic.position.start);
    const column = diagnostic.position.start - lineSplits[line - 1] + 1;
    // Unfortunately don't get to set an end point, level or any labels
    plugin.warn(diagnostic.label, { line, column })
  }
}

export default createUnplugin((_options) => {
  return {
    name: "decaf",
    transformInclude(id) {
      const extension = id.split(".").at(-1);
      return ["ts", "tsx", "js", "jsx"].includes(extension);
    },
    transform(code, id) {
      function resolver(path) {
        if (path !== id) {
          console.error(`tried to read another path '${path}' which is currently unsupported by the plugin`)
          return "ERROR";
        } else {
          return code;
        }
      }
      const output = build(resolver, id);
      if (output.Ok) {
        emitDiagnostics(code, output.Ok.temp_warnings_and_infos, this)
        return {
          code: output.Ok.outputs[0].content
        }
      } else {
        emitDiagnostics(code, output.Err, this)
        this.warn("decaf had errors and did not transform");
        return code;
      }
    },
  };
});
