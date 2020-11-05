<br />
<p align="center">
  <a href="https://github.com/littledivy/deno_sass">
    <img src="./assets/deno_sass.png" alt="deno_sass logo" width="310">
  </a>
  <h3 align="center">deno_sass</h3>

  <p align="center">
    Wasmified version of the near-feature-complete Grass Sass compiler for Deno.
 </p>
 <p align="center">

  [![stars](https://img.shields.io/github/stars/littledivy/deno_sass2)](https://github.com/littledivy/deno_sass2/stargazers)
  [![issues](https://img.shields.io/github/issues/littledivy/deno_sass2)](https://github.com/littledivy/deno_sass2/issues)
 </p>
</p>

## Example

```typescript
import { compile } from "https://deno.land/x/sass/mod.ts";

compile("a { color: #000; }");
```

## Building from source

### Prerequisites

- [deno](https://deno.land/)
- [rust](https://www.rust-lang.org/)
- wasm-pack

## Building
```bash
$ deno run -A build.js
```

## Example

```bash
$ deno run example.ts
```

### Contribution

Pull request, issues and feedback are very welcome. Code style is formatted with `deno fmt` and commit messages are done following [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) spec.

## Copyright

deno_sass is licensed under the MIT license. Please see the [LICENSE](LICENSE) file.
