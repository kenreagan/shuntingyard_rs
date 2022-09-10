#[derive(Debug, PartialOrd, PartialEq)]
enum Operator{
    Mul,
    Add,
    Sub,
    Div
}

#[derive(Debug, PartialOrd, PartialEq)]
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
    fn parser<T: AsRef<str>>(expr: T)->Vec<SyntaxTree>{
        let expr = expr.as_ref();
        let mut tokens: Vec<_> = Vec::new();
        let chars = expr.chars();

        for objects in chars{
            match objects {
                '0'..='9' => {
                    match tokens.last_mut(){
                    Some(SyntaxTree::Num(n)) => {
                        *n = *n * 10 +(objects as u32 - 48);
                    },
                    _ =>{
                            let num = objects as u32 - 48;
                            tokens.push(SyntaxTree::Num(num))
                        }
                    }
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
        
        };

        tokens
    
    }
    

    fn expression(mut tokens: Vec<SyntaxTree>)-> Vec<SyntaxTree>{
    // Create Operator stack and Digit stack
        tokens.reverse();


        let mut queue: Vec<SyntaxTree> = Vec::new();

        let mut stack: Vec<SyntaxTree> = Vec::new();


        while let Some(token) = tokens.pop() {
        
            match(token){
                SyntaxTree::Num(_) => queue.push(token),
                SyntaxTree::Sign(_) => {
                    while !stack.is_empty() && stack[stack.len() -1] >= token{
                        queue.push(stack.pop().unwrap());
                    }
                    stack.push(token);
                },
                _ =>{}
            
            }
        }

        while stack.len() > 0 {
            queue.push(stack.pop().unwrap())
        }

        queue
    }

   fn evaluate(tokens:&mut Vec<SyntaxTree>)->Option<f32>{
      tokens.reverse();

      let mut stack: Vec<f32> = Vec::new();


      while let Some(token) = tokens.pop() {
          match token{
            SyntaxTree::Num(token) => {
                stack.push(token as f32)
            },
            SyntaxTree::Sign(Operator::Add) => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();

                stack.push(left + right)
            },
            SyntaxTree::Sign(Operator::Mul) => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();

                stack.push(left * right)
            },
            SyntaxTree::Sign(Operator::Div) => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();

                stack.push(left / right)
            },
            SyntaxTree::Sign(Operator::Sub) => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();

                stack.push(left - right)
            },
            _ => {}
          }
      }


      if stack.len() >1 {
        None
      }else{
        stack.pop()
      }
   }
}



#[test]
fn test_addition(){
    let expr = "10 + 4".to_string();

    let res = Loader::evaluate(&mut Loader::expression(Loader::parser(&expr)));
    assert_eq!(res, Some(14.0))
}


#[test]
fn test_mul() {
    let expr = "4 * 4".to_string();
    let res = Loader::evaluate(&mut Loader::expression(Loader::parser(&expr)));

    assert_eq!(res, Some(16.0))
}

fn main(){

    loop{
        let mut expression = String::new();
        match std::io::stdin().read_line(&mut expression){
            Ok(_) =>{
                let token = Loader::parser(&expression);
                let mut expr = Loader::expression(token);

                if let Some(results) = Loader::evaluate(&mut expr){
                    println!("{}", results);
                }
            },
            Err(_) =>{
                println!("Error")
            }
        }
    }
}
