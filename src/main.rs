#[derive(Debug)]
enum Operator{
    Mul,
    Add,
    Sub,
    Div
}

#[derive(Debug)]
enum SyntaxTree{
    Num(u32),
    Sign(Operator),
}

#[derive(Debug)]
#[allow(unused)]
enum Error{
    LogicalErr
}

struct Loader{}

#[allow(unused)]
impl Loader{
    fn parser(expr: &str)->Vec<SyntaxTree>{

        let mut tokens: Vec<_> = Vec::new();
        let chars = expr.chars();

        for objects in chars{
            match objects {
                '0'..='9' =>{
                    tokens.push(SyntaxTree::Num(objects as u32))
                },
                '+' =>{
                    tokens.push(SyntaxTree::Sign(Operator::Add))
                },
                '/' => {
                    tokens.push(SyntaxTree::Sign(Operator::Div))
                },
                '*' => {
                    tokens.push(SyntaxTree::Sign(Operator::Mul))
                }
                '-' => {
                    tokens.push(SyntaxTree::Sign(Operator::Sub))
                },
                '\n' => {},
                ' ' => {},
                _ => {}
            }
        }

        tokens
    
    }


   // fn evaluate()->{
    //
   // }/
}



fn main(){
    let mut expression = String::new();
    match std::io::stdin().read_line(&mut expression){
        Ok(_) =>{
             let expression = Loader::parser(&expression);
             println!("{:?}", expression);
        },
        Err(_) =>{
            println!("Error")
        }
    }    
}
