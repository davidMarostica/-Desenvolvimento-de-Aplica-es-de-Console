mod tela;
mod models;

use tela::menu as menu;
use crate::models::cliente;

fn main() {
    let mut clientes: Vec<cliente::Cliente> = Vec::new();
    menu::mostra_menu(&mut clientes); // passa o vetor como argumento
}
