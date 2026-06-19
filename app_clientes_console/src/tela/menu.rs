use crate::tela::ler;

pub fn mostra_menu() {
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

        let opcao: i32 = ler ::ler_dados_int();
        match opcao {
            1 => println!("Opção 1 selecionada: Cadastrar cliente"),
            2 => println!("Opção 2 selecionada: Excluir cliente"),
            3 => println!("Opção 3 selecionada: Listar clientes"),
            0 => {
                println!("Saindo do programa...");
                return;
            }
            _ => println!("Opção inválida, tente novamente."),
        }

        print!("Pressione Enter para continuar...");
        ler::ler_dados(); // Pausa para o usuário ler a mensagem antes de mostrar o menu novamente

        
    }
}
