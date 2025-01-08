import { $ } from "zx";
import { run, bench, summary, barplot } from 'mitata';

const TSC_PATH = "./node_modules/.bin/tsc";
const DECAF_PATH = "../target/release/decaf";

await $`mkdir -p ./tmp`;

await $`cargo build --release`;
const tsc_version = await $`${TSC_PATH} --version`;
const decaf_version = await $`${DECAF_PATH} info | head -n 1`;
console.log('');
console.log(`tsc version: ${tsc_version}`);
console.log(`decaf version: ${decaf_version}`);

await $`cat ../checker/specification/specification.md ../checker/specification/staging.md > ./tmp/simple.md`;
await $`cargo run -p decaf-parser --example code_blocks_to_script ./tmp/simple.md --comment-headers --out ./tmp/simple.tsx`;
await $`cat ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md > ./tmp/middle.md`;
await $`cargo run -p decaf-parser --example code_blocks_to_script ./tmp/middle.md --comment-headers --out ./tmp/middle.tsx`;
await $`cat ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md > ./tmp/complex.md`;
await $`cargo run -p decaf-parser --example code_blocks_to_script ./tmp/complex.md --comment-headers --out ./tmp/complex.tsx`;
await $`cat ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md ./tmp/simple.md > ./tmp/very_complex.md`
await $`cargo run -p decaf-parser --example code_blocks_to_script ./tmp/very_complex.md --comment-headers --out ./tmp/very_complex.tsx`;

summary(() => {
  bench('tsc --noEmit simple.tsx', async () => {
    try {
      await $`${TSC_PATH} --noEmit ./tmp/simple.tsx`;
    } catch { }
  });

  bench('decaf check simple.tsx', async () => {
    try {
      await $`${DECAF_PATH} check ./tmp/simple.tsx`.quiet();
    } catch { }
  });
});

summary(() => {
  bench('tsc --noEmit middle.tsx', async () => {
    try {
      await $`${TSC_PATH} --noEmit ./tmp/middle.tsx`;
    } catch { }
  });

  bench('decaf check middle.tsx', async () => {
    try {
      await $`${DECAF_PATH} check ./tmp/middle.tsx`.quiet();
    } catch { }
  });
});

summary(() => {
  bench('tsc --noEmit complex.tsx', async () => {
    try {
      await $`${TSC_PATH} --noEmit ./tmp/complex.tsx`;
    } catch { }
  });

  bench('decaf check complex.tsx', async () => {
    try {
      await $`${DECAF_PATH} check ./tmp/complex.tsx`.quiet();
    } catch { }
  });
});

summary(() => {
  bench('tsc --noEmit very_complex.tsx', async () => {
    try {
      await $`${TSC_PATH} --noEmit ./tmp/very_complex.tsx`;
    } catch { }
  });

  bench('decaf check very_complex.tsx', async () => {
    try {
      await $`${DECAF_PATH} check ./tmp/very_complex.tsx`.quiet();
    } catch { }
  });
});

barplot(() => {
  bench('tsc --noEmit simple.tsx', async () => {
    try {
      await $`${TSC_PATH} --noEmit ./tmp/simple.tsx`;
    } catch { }
  });

  bench('tsc --noEmit middle.tsx', async () => {
    try {
      await $`${TSC_PATH} --noEmit ./tmp/middle.tsx`;
    } catch { }
  });

  bench('tsc --noEmit complex.tsx', async () => {
    try {
      await $`${TSC_PATH} --noEmit ./tmp/complex.tsx`;
    } catch { }
  });

  bench('tsc --noEmit very_complex.tsx', async () => {
    try {
      await $`${TSC_PATH} --noEmit ./tmp/very_complex.tsx`;
    } catch { }
  });
});

barplot(() => {
  bench('decaf check simple.tsx', async () => {
    try {
      await $`${DECAF_PATH} check ./tmp/simple.tsx`.quiet();
    } catch { }
  });

  bench('decaf check middle.tsx', async () => {
    try {
      await $`${DECAF_PATH} check ./tmp/middle.tsx`.quiet();
    } catch { }
  });

  bench('decaf check complex.tsx', async () => {
    try {
      await $`${DECAF_PATH} check ./tmp/complex.tsx`.quiet();
    } catch { }
  });

  bench('decaf check very_complex.tsx', async () => {
    try {
      await $`${DECAF_PATH} check ./tmp/very_complex.tsx`.quiet();
    } catch { }
  });
})

await run();
