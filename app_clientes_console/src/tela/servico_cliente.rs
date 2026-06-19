use crate::{models::cliente::Cliente, tela::{ler::ler_dados, operacoes_basica::limpar_tela}};

pub fn incluir_cliente(clientes: &mut Vec<Cliente>) {
        limpar_tela();

    let mut cliente: Cliente = Cliente::default();
    cliente.id = clientes.len() + 1;

    println!("Digite o nome do cliente:");
    cliente.nome = ler_dados();

    println!("Digite o CPF do cliente:");
    cliente.cpf = ler_dados();

    println!("Digite o endereço do cliente:");
    cliente.endereco = ler_dados();

    clientes.push(cliente);
    println!("Cliente cadastrado com sucesso!");
}

pub fn excluir_cliente(clientes: &mut Vec<Cliente>) {
    limpar_tela();

    println!("Digite o ID do cliente a excluir:");
    let id: usize = ler_dados().parse().unwrap_or(0);

    if let Some(pos) = clientes.iter().position(|c| c.id == id) {
        clientes.remove(pos);
        println!("Cliente removido com sucesso!");
    } else {
        println!("Cliente não encontrado.");
    }
}

pub fn listar_clientes(clientes: &Vec<Cliente>) {
    limpar_tela();

    if clientes.is_empty() {
        println!("Nenhum cliente cadastrado.");
    } else {
        println!("Lista de clientes:");
        for cliente in clientes {
            println!(
                "ID: {} | Nome: {} | CPF: {} | Endereço: {}",
                cliente.id, cliente.nome, cliente.cpf, cliente.endereco
            );
        }
    }
}
