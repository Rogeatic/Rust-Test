fn main(){
    println!("hello world");
    let maxAmount: u8 = 50;

    let name: &str = "James";
    let mut income: u8 = 5;
    
    //println!("{}", name);

    while income < maxAmount{
        income += 2;
    }


    println!("{}", returnSentance(name, income));

}

pub fn returnSentance(name: &str, amount: u8) -> &str {
    let mut sentance: &str = "";
    println!("Name: {}    Amount of Money: ${}", name, amount);

    return (sentance);
}