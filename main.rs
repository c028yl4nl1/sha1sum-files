use glob::glob;
use sha1::{Digest, Sha1};
use std::collections::{HashMap, HashSet};
use std::env::args;
use std::path::PathBuf;
use std::process::exit;
use std::thread::spawn;
// Construtor
struct Path {
    caminho: String,
}
impl Path {
    pub fn new(caminho: String) -> Path {
        Path { caminho }
    }
}

//Construtor
fn main() {
    let mut dados: HashMap<String, std::path::PathBuf> = HashMap::new();
    let arg = argumentos(); // argumentos > env < Variavel de ambiente

    for arquivos in glob(&arg.unwrap().caminho).unwrap() {
        if let Ok(valor_caminho) = arquivos {
            if let Ok(buffer_arquivo) = abrir_arquivo(&valor_caminho) {
                // vou chamar o sha1
                let sha1_hash = calcular_sha1_sum(buffer_arquivo);

                if dados.contains_key(&sha1_hash) {
                    println!(
                        "O arquivo: {:?} é igual o {:?}",
                        valor_caminho,
                        dados.get(&sha1_hash).unwrap()
                    );
                } else {
                    dados.insert(sha1_hash, valor_caminho);
                }
            }
        }
    }

    for (hash, caminho) in dados {
        println!("Hash <{}> / path <{}>", hash, caminho.display());
    }
}

fn argumentos() -> Result<Path, std::process::ExitCode> {
    let mut vector_String_var: Vec<String> = args().collect::<Vec<String>>();

    if vector_String_var.len() < 2 {
        println!("Please arquivo or Diretorio");
        return exit(1); // ExitCode impl exit
    } else {
        return Ok(Path {
            caminho: vector_String_var[1].clone(),
        });
    }
}

fn calcular_sha1_sum(arquivo: String) -> String {
    let mut sha1_consult = Sha1::new(); // key ?
    sha1_consult.update(arquivo);

    let hexsha1_sum = sha1_consult.finalize();

    format!("{:0x}", hexsha1_sum)
}

fn abrir_arquivo(arquivo_caminho: &PathBuf) -> Result<String, &'static str> {
    use std::fs::read_to_string;

    if let Ok(buffer_Arquivo) = read_to_string(arquivo_caminho) {
        Ok(buffer_Arquivo)
    } else {
        Err("Error ao abrir arquivo")
    }
}
