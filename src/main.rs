use sha2::{Sha256, Digest};
use std::fs::{self, OpenOptions};
use std::io::{self, Write, Read};
use std::path::Path;
use reqwest::blocking::get;
use std::time::{SystemTime, UNIX_EPOCH};
use base64::{engine::general_purpose, Engine as _};
use rpassword::read_password;
use std::process::exit;
use serde::{Serialize, Deserialize};
use serde_json::{self, Value};
use uuid::Uuid;

const DB_PATH: &str = "usuarios.json";
const LOG_PATH: &str = "imagem.log";

#[derive(Serialize, Deserialize, Clone)]
struct Usuario {
    nome_hash: String,
    senha_hash: String,
    imagem_id: String,
}

fn main() {
    loop {
        println!("\nSelecione uma opção:");
        println!("1. Criar usuário");
        println!("2. Listar todos os usuários");
        println!("3. Editar usuário");
        println!("4. Validar autenticação usuário");
        println!("5. Sair");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => criar_usuario(),
            "2" => listar_usuarios(),
            "3" => editar_usuario(),
            "4" => validar_usuario(),
            "5" => exit(0),
            _ => println!("Opção inválida."),
        }
    }
}

fn gerar_hash_sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hex::encode(hasher.finalize())
}

fn gerar_chave_da_imagem(imagem_id: &str) -> Vec<u8> {
    let url = format!("https://picsum.photos/id/{}/200", imagem_id);
    let response = get(&url).unwrap();
    let bytes = response.bytes().unwrap();
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    hasher.finalize().to_vec()
}

fn log_imagem(imagem_id: &str, hash: &[u8]) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_PATH)
        .unwrap();
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    writeln!(file, "{} - ID {} - {}", timestamp, imagem_id, general_purpose::STANDARD.encode(hash)).unwrap();
}

fn criptografar_lavalamp(senha: &str, chave: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(chave);
    hasher.update(senha.as_bytes());
    hex::encode(hasher.finalize())
}

fn carregar_usuarios() -> Vec<Usuario> {
    if !Path::new(DB_PATH).exists() {
        return vec![];
    }
    let dados = fs::read_to_string(DB_PATH).unwrap();
    serde_json::from_str(&dados).unwrap_or_default()
}

fn salvar_usuarios(usuarios: &Vec<Usuario>) {
    let dados = serde_json::to_string_pretty(usuarios).unwrap();
    fs::write(DB_PATH, dados).unwrap();
}

fn criar_usuario() {
    println!("Digite o nome do usuário:");
    let mut nome = String::new();
    io::stdin().read_line(&mut nome).unwrap();
    let nome_hash = gerar_hash_sha256(nome.trim());

    println!("Digite a senha:");
    let senha = read_password().unwrap();

    let imagem_id = Uuid::new_v4().to_string();
    let chave = gerar_chave_da_imagem(&imagem_id);
    log_imagem(&imagem_id, &chave);
    let senha_hash = criptografar_lavalamp(&senha, &chave);

    let mut usuarios = carregar_usuarios();
    if usuarios.iter().any(|u| u.nome_hash == nome_hash) {
        println!("Usuário já existe.");
        return;
    }

    usuarios.push(Usuario {
        nome_hash,
        senha_hash,
        imagem_id,
    });
    salvar_usuarios(&usuarios);
    println!("Usuário criado com sucesso.");
}

fn listar_usuarios() {
    let usuarios = carregar_usuarios();
    if usuarios.is_empty() {
        println!("Nenhum usuário cadastrado.");
    } else {
        for (i, user) in usuarios.iter().enumerate() {
            println!("{}: Nome hash: {} | Senha: {} | ID Imagem: {}", i + 1, user.nome_hash, user.senha_hash, user.imagem_id);
        }
    }
}

fn editar_usuario() {
    println!("Digite o nome do usuário:");
    let mut nome = String::new();
    io::stdin().read_line(&mut nome).unwrap();
    let nome_hash = gerar_hash_sha256(nome.trim());

    let mut usuarios = carregar_usuarios();
    if let Some(index) = usuarios.iter().position(|u| u.nome_hash == nome_hash) {
        println!("Digite a senha atual:");
        let senha = read_password().unwrap();
        let chave_antiga = gerar_chave_da_imagem(&usuarios[index].imagem_id);
        let senha_hash = criptografar_lavalamp(&senha, &chave_antiga);

        if senha_hash != usuarios[index].senha_hash {
            println!("Senha incorreta.");
            return;
        }

        println!("Digite o novo nome:");
        let mut novo_nome = String::new();
        io::stdin().read_line(&mut novo_nome).unwrap();
        let novo_nome_hash = gerar_hash_sha256(novo_nome.trim());

        println!("Digite a nova senha:");
        let nova_senha = read_password().unwrap();

        let nova_imagem_id = Uuid::new_v4().to_string();
        let nova_chave = gerar_chave_da_imagem(&nova_imagem_id);
        log_imagem(&nova_imagem_id, &nova_chave);

        usuarios[index] = Usuario {
            nome_hash: novo_nome_hash,
            senha_hash: criptografar_lavalamp(&nova_senha, &nova_chave),
            imagem_id: nova_imagem_id,
        };

        salvar_usuarios(&usuarios);
        println!("Usuário atualizado com sucesso.");
    } else {
        println!("Usuário não encontrado.");
    }
}

fn validar_usuario() {
    println!("Digite o nome do usuário:");
    let mut nome = String::new();
    io::stdin().read_line(&mut nome).unwrap();
    let nome_hash = gerar_hash_sha256(nome.trim());

    let usuarios = carregar_usuarios();
    if let Some(user) = usuarios.iter().find(|u| u.nome_hash == nome_hash) {
        println!("Digite a senha:");
        let senha = read_password().unwrap();
        let chave = gerar_chave_da_imagem(&user.imagem_id);
        log_imagem(&user.imagem_id, &chave);
        let senha_hash = criptografar_lavalamp(&senha, &chave);

        if senha_hash == user.senha_hash {
            println!("Autenticação válida!");
        } else {
            println!("Senha inválida.");
        }
    } else {
        println!("Usuário não encontrado.");
    }
}
