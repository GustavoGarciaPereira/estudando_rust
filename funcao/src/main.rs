fn retorna_um(a:i32,b:i32) -> i32{
    a+b
}

fn funcao_generica<T>(qualquer_coisa:T) -> T{
    qualquer_coisa
}

fn main() {
    assert_eq!(funcao_generica(2), 2);
    assert_eq!(funcao_generica(true), true);
    assert_eq!(funcao_generica("guguzinho"), "guguzinho");

    assert_eq!(retorna_um(1,2),3);
    println!("{}",retorna_um(2,3));
    println!("Hello, world!");
}
