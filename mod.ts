import init, {
  compile as wasm_compile,
  source
} from "./wasm.js";

await init(source);

export function compile(source: string): string {
  return wasm_compile(source, {});
}

console.log(compile("a { color: #fff; }"));

