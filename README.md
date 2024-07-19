# orch

Orch (stands for "orchestrator") is a library for building LLM-powered applications and agents for the Rust programming language.  
It was primarily built for usage in [magic-cli](https://github.com/guywaldman/magic-cli), but can be used in other contexts as well.

> [!NOTE]
>
> If the project gains traction, this can be compiled as an addon to other languages such as Python or a standalone WebAssembly module.

There is currently support for text generation with `ollama` (either stream or non-stream).  
Originally this contained agents and tools as well, but this was removed for now.

## Roadmap

- [ ] Support for text generation with `openai`
- [ ] Embedding generation
- [ ] Agents and tools