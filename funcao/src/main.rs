fn retorna_um(a:i32,b:i32) -> i32{
    a+b
}

fn funcao_generica<T>(qualquer_coisa:T) -> T{
    qualquer_coisa
}

//funcao recenbendo funcao
fn decorador<F>(func:F, n:i32)->i32
where F:Fn(i32) -> i32{
    func(n)
}


fn main() {

    fn sub(num:i32) -> i32{
        num-10
    }

    let a = |x| x*2;
    println!("{}",a(23));

    assert_eq!(funcao_generica(2), 2);
    assert_eq!(funcao_generica(true), true);
    assert_eq!(funcao_generica("guguzinho"), "guguzinho");

    assert_eq!(retorna_um(1,2),3);

    assert_eq!(decorador(sub,42),32);
    println!("<decorador-> {}",decorador(sub,50));

    
    println!("{}",retorna_um(2,3));
    println!("Hello, world!");
}
