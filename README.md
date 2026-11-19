# 🦀 Rustlings PT-BR

Tradução em **Português Brasileiro** dos exercícios oficiais do [Rustlings](https://github.com/rust-lang/rustlings) - pequenos exercícios práticos para aprender a ler e escrever código em **Rust**.

[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/BeemerCodes/Rustling-PT-BR/pulls)
[![Rust](https://img.shields.io/badge/Rust-2026%20Edition-orange.svg)](https://www.rust-lang.org/pt-BR)

> Projeto mantido de forma aberta com o objetivo de oferecer à comunidade brasileira um material atualizado de prática em Rust, complementando o [Rust Book PT-BR](https://github.com/BeemerCodes/Rust-book-PT-BR).

---

## 📋 Pré-requisitos

Antes de começar, certifique-se de ter o **Rust** e o **Cargo** instalados (via [rustup](https://rustup.rs/)), e instale a CLI do `rustlings`:

```bash
cargo install rustlings
```

---

## 🚀 Como usar

1. **Clone este repositório:**

   ```bash
   git clone https://github.com/BeemerCodes/Rustling-PT-BR.git
   cd Rustling-PT-BR
   ```

2. **Inicie o modo interativo do Rustlings:**

   ```bash
   rustlings
   ```

3. **Resolva os exercícios:**
   - O Rustlings vai indicar o arquivo do exercício atual em `exercises/`.
   - Abra o arquivo no seu editor favorito, corrija os erros ou complete os `TODO`s e salve — o terminal recompila e verifica automaticamente.

### Atalhos úteis no terminal do Rustlings

| Tecla | Ação |
| :---: | :--- |
| `h` | Exibe uma dica (*hint*) para o exercício atual |
| `n` | Avança para o próximo exercício |
| `l` | Lista todos os exercícios e seu progresso |
| `r` | Reinicia o exercício atual |
| `q` | Sai do Rustlings |

---

## 📂 Estrutura do projeto

- `exercises/` — Todos os exercícios traduzidos para PT-BR, organizados por tema (e com [mapeamento para os capítulos do livro](exercises/README.md)).
- `solutions/` — Soluções de referência para cada exercício.
- `Cargo.toml` — Configuração dos binários e lints do projeto.
- `rust-analyzer.toml` — Configuração recomendada para o `rust-analyzer` na sua IDE.

---

## 🤝 Como contribuir

Contribuições são muito bem-vindas! Se você encontrou algum erro de tradução, termo que pode ficar mais natural em português ou exercício desatualizado:

1. Faça um **Fork** do projeto
2. Crie uma branch para sua alteração (`git checkout -b fix/traducao-structs`)
3. Faça o commit das mudanças (`git commit -m "fix: melhora tradução em structs1"`)
4. Envie para o seu fork (`git push origin fix/traducao-structs`)
5. Abra um **Pull Request**
