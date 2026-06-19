# app_clientes_console

Aplicação de console em Rust para gerenciamento básico de clientes.

## Descrição

Este projeto implementa um sistema simples de cadastro, exclusão e listagem de clientes via terminal. Cada cliente possui:
- ID
- Nome
- CPF
- Endereço

Os dados são mantidos apenas em memória durante a execução do programa.

## Estrutura do projeto

- `app_clientes_console/Cargo.toml` - configurações do pacote e dependências
- `app_clientes_console/src/main.rs` - ponto de entrada da aplicação
- `app_clientes_console/src/models/cliente.rs` - definição da struct `Cliente`
- `app_clientes_console/src/tela/menu.rs` - menu de interação
- `app_clientes_console/src/tela/ler.rs` - leitura de dados do usuário
- `app_clientes_console/src/tela/operacoes_basica.rs` - limpeza de tela e espera
- `app_clientes_console/src/tela/servico_cliente.rs` - operações de inclusão, exclusão e listagem

## Requisitos

- Rust 1.80+ (ou compatível com a edição 2024)
- Cargo

## Como compilar

No diretório raiz do repositório:

```bash
cargo build --manifest-path app_clientes_console/Cargo.toml
```

## Como executar

```bash
cargo run --manifest-path app_clientes_console/Cargo.toml
```

## Uso

Ao iniciar, escolha uma opção do menu:

1. Cadastrar cliente
2. Excluir cliente
3. Listar clientes
0. Sair

> Atenção: os dados não são persistidos em disco. Ao fechar o programa, os registros são perdidos.

## Dependências

- `clearscreen` - limpa a tela do terminal entre as operações

## Licença

Use conforme necessário. Este é um projeto educacional de demonstração.

## Autor
David Aparecido da silva 

GitHub: @davidMarostica

Linkdin: https://www.linkedin.com/in/david-aparecido-da-silva/