# 🔥 Rust LavaLamp Project

Um projeto educacional de criptografia inspirado no sistema **LavaRand** da Cloudflare, desenvolvido em **Rust**, com interface via terminal.

---

## 🌋 Inspiração: ClaudeFlare LavaRand

A Cloudflare utiliza um sistema físico de **lâmpadas de lava** chamado **LavaRand** para gerar entropia real e aleatória a partir das imagens das lâmpadas. Essas imagens são processadas em tempo real para alimentar os sistemas de geração de chaves criptográficas, garantindo segurança contra previsibilidade e ataques determinísticos.

Este projeto traz uma **versão conceitual e educacional** dessa ideia usando a API [Picsum.photos](https://picsum.photos), que oferece imagens aleatórias a cada requisição, simulando a imprevisibilidade das lâmpadas de lava.

---

## 🧠 Objetivo

Este projeto foi criado como forma de **praticar conceitos de criptografia**, manipulação de hashes, segurança de autenticação e gerenciamento seguro de senhas, com foco no aprendizado da disciplina de **Criptografia e Segurança de Sistemas**.

---

## ⚙️ Como Funciona

### 🔐 Cadastro de Usuários

Cada usuário é cadastrado com os seguintes dados:

- **Nome (hash SHA256)**: utilizado como identificador único
- **Senha (criptografada)**: combinando a senha com uma chave derivada de uma imagem aleatória
- **ID da imagem**: representa a imagem usada como origem da entropia

### 🖼️ Geração de chave

- Uma imagem aleatória é baixada via `https://picsum.photos/id/{ID}/200`
- A imagem é transformada em bytes
- Esses bytes são processados com `SHA256` para gerar uma **chave única**
- A senha do usuário é misturada com essa chave e também processada com `SHA256`, produzindo a **senha final criptografada**

---

## 🖥️ Funcionalidades da Interface

- `Criar usuário`: pede nome e senha, gera imagem, cria hash e armazena
- `Listar todos os usuários`: mostra nome (hash), senha (hash) e ID da imagem
- `Editar usuário`: exige senha atual, permite alterar nome e senha
- `Validar autenticação`: verifica nome + senha com base na imagem original
- `Sair`: encerra o programa

---

## 📁 Estrutura de Arquivos

- `usuarios.json`: banco de dados com os usuários
- `imagem.log`: log das imagens utilizadas e suas hashes
- `src/main.rs`: lógica principal do projeto
- `Cargo.toml`: dependências do projeto

---

## 🛠️ Tecnologias

- Linguagem: **Rust**
- Criptografia: `sha2`, `base64`, `hex`
- Input seguro: `rpassword`
- API de imagens: `reqwest` com [Picsum.photos](https://picsum.photos)
- Serialização: `serde`, `serde_json`
- UUID: para gerar IDs únicos de imagens

---

## 📚 Finalidade

Este projeto não tem finalidade de produção ou segurança real. Seu propósito é **educacional**, promovendo a experimentação com conceitos de:

- Geração de chaves dinâmicas
- Criptografia simétrica
- Hashing seguro (SHA256)
- Autenticação baseada em hash + fator externo (imagem)

Ideal para estudantes e entusiastas de segurança da informação que queiram aplicar teoria na prática com Rust.

---

## 🤝 Contribuição

Sugestões, melhorias ou testes são bem-vindos! Fork, clone e experimente.

---

## 🧠 Créditos

Projeto idealizado com base na ideia do sistema **LavaRand** da Cloudflare.

Desenvolvido como prática da disciplina de **Criptografia**, por _Samuel_.
