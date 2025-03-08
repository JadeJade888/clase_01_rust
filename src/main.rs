


fn main() {
    //variables
    let x = 5;
    //x=10; esto no es valido por la variabla no tiene mutabilidad

    let mut y = 10;
    y=20;

    //shadowing o sombreado
    let z =5;
    {
        //creando un ambiente con {}
        let z=z+1; //nueva variable
        println!("el valor de z {}",z);
    }
    let entero: i32 = 42;
    let flotante: f64 =3.1416;
    let booleano: bool =true;
    let caracter: char = 'a';
    //tupla -> structura, creacion de tupla ()
    let tupla:(i32, f64, char)=(30, 5.65, 'b');
    //arreglo []
    let arreglo: [i32; 3]=[1,2,3];


    println!("el valor de x {}",x);
    println!("el valor de y {}",y);
    println!("el valor de z {}",z);
    println!("el valor de tupla {:?}",tupla);
    println!("tupla:({},{},{})",tupla.0,tupla.1,tupla.2);
    println!("el valor de arreglo {:?}",arreglo);
    println!("Hello, world!");
}
