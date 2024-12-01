export type ReadFromFS = (path: string) => string | null;

export interface DecafUnpluginOptions {
    /** Defaults to only running on .decaf.* files */
    allJsTsFiles?: boolean,

    customBuild?: (cb: ReadFromFS, entryPath: string, minify: boolean) => any
}
