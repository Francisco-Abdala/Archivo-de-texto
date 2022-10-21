use std::fs;
use std::fs::File;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;

// si lo ejecuta n veces, tendrá n veces el contenido en el archivo creado y si es la primera vez que lo ejecuta,crea el archivo nuevo
// en el documento no especifica si tengo que aplicar el algoritmo de popularidad con la canción agregada

fn create_blank_file(p: &Path){
    let _file = File::create(p).expect("El archivo no pudo crearse");
    println!("El archivo fue creado");
    // crea archivo en blanco en caso de que no exista

}


fn add_new_content(mut f: &File, texto : &String, str : &str, salto_de_linea : &str){
    //añade contenido al archivo creado o archivo existente

    f.write_all(str.as_bytes());
    f.write_all(salto_de_linea.as_bytes());
    f.write_all(texto.as_bytes()).ok();
}

fn open_file_to_append(p: &Path) -> File{


    let mut binding = OpenOptions::new();
    let binding = binding.append(true);
    let file = match binding.open(p){
        Err(_why) => panic!("No se puede abrir el archivo"),
        Ok(file) => file,
    };

    //add_new_content(&file);

    return file

}

fn open_file_to_read(p: &Path){
    //comprueba si el archivo existe o no
    if Path::new(p).exists(){
        let _file = match File::open(&p){
            Err(_why) => panic!("El archivo no se puede abrir..."),
            Ok(file) => file,
        };

    } else {
        create_blank_file(p);
    }
}



fn main(){
    let str = fs::read_to_string("top50.csv").expect("No se pudo leer el archivo.");
    println!("Este es el contenido del archivo: \n {}",str);


    let mut array : [[&str;5];52] = [["";5];52];
    let mut linea = 0;
    for x in str.split("\n"){
        let mut contador = 0;
        for y in x.split(","){
            if contador < 5{
                array[linea][contador] = y.trim();
            }
            contador += 1;
        }
        linea += 1;
    }
    let mut mayor_popularidad = "";
    let mut menor_popularidad = array[1][4];


    for i in 1..51{
        
        if array[i][4] > mayor_popularidad{
            mayor_popularidad = array[i][4];
        } if array[i][4] < menor_popularidad{
            menor_popularidad = array[i][4];

        }
    }


        let mut mejor_cancion = "";

        let mut peor_cancion = "";
        for i in 0..51{
            if array[i][4] == mayor_popularidad{
                mejor_cancion = array[i][1];
            } if array[i][4] == menor_popularidad{
                peor_cancion = array[i][1];
            }
        
        }
        let mut informacion_1 = String::from("");
        let mut informacion_2 = String::from("");
        let mj_cn = String::from(mejor_cancion);
        let my_p =String::from( mayor_popularidad);
        let pr_cn = String::from(peor_cancion);
        let mn_p = String::from( menor_popularidad);
        let espacio = String::from(" ");


        informacion_1 = mj_cn + &espacio + &my_p;
        informacion_2 = pr_cn + &espacio + & mn_p;


        println!("La canción más popular, junto a su puntaje es: {}",informacion_1);
        println!("La canción menos popular, junto a su puntaje es: {}", informacion_2);

        
        let texto: String = "51, Aquella noche, Bardero$, trap urbano,97".to_string();
        let path = Path::new("top51.csv");
        let salto_de_linea = "\n";
        open_file_to_read(path);
        let file = open_file_to_append(path);
        add_new_content(&file, &texto, &str,salto_de_linea);
       
    }


        
   

