use crate::models::cliente::Cliente;
use crate::tela::ler::*;
use crate::tela::operacoes_basica::{esperar};
use crate::tela::servico_cliente::{incluir_cliente, excluir_cliente, listar_clientes};

pub fn mostra_menu(clientes: &mut Vec<Cliente>) {
    
    loop {
        println!(
            "============================Menu=============================\n\
             Escolha uma opção:\n\n\
             1. Cadastrar cliente\n\
             2. Excluir cliente\n\
             3. Listar clientes\n\
             0. Sair\n\
             ============================================================="
        );

        let opcao: i32 = ler_dados_int();

        match opcao {
            1 => incluir_cliente(clientes),
            2 => excluir_cliente(clientes),
            3 => listar_clientes(clientes),
            0 => {
                println!("Saindo do programa...");
                return;
            }
            _ => println!("Opção inválida, tente novamente."),
        }

        esperar(2);
    }
}
