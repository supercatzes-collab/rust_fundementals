use crate::helper_functions::{get_f64, get_i32, get_input_string};

mod helper_functions;

fn main() {
    //Declares the account variable early without any value.
    //"Optio<T>" is an enum with two types. Where Some(T) exists or None. It can also return the value the exists.
    let mut account:Option<Account> = None;

   loop {
     let input: i32 = get_i32("new account [1] deposit[2] withdraw[3] exit[0]: ");
     match input {
        0 => return,
            //Some() returns the data that was called back to the account variable outside of match transferring ownership back to it.
        1 => account = Some(create_account()),
        2 => match deposit(&mut account) {
                Ok(msg) => println!("{}", msg),
                Err(e) => println!("{}", e),
            },
        3 => match withdraw(&mut account) {
                Ok(msg) => println!("{}", msg),
                Err(e) => println!("error: {}", e),
            },
        _ => println!("end."),
    }
   }
}
//struct for the structure of the Account variable
struct Account {
    owner: String,
    balance: f64,
}

//Gets the account name then asks for deposit ammount. 
//The it calls the Impl function to store the data into the struct.
fn create_account() -> Account {
    let input: String = get_input_string("account name: ");
    let deposit: f64 = get_f64("enter deposit amount: ");

    Account::new(input, deposit)
}

//impl is a constructor that you attach to compound/trait types inorder to perform functions like how you'd normally do inside functions.
impl Account {
    //fn new takes two arguments: string and a f64 value. Then it creates a new struct instance using those values then returns it to whoever calls it using X::new() via -> Self.
    //The reason that "Self" is capitalized in "fn new" is because this function is referring to the blueprint. I.e what the impl is based on, in this example the struct.
    //If your function needs to manipulate the blueprint specifically and not an instance of it, then you'd used the capitalized "Self". But i don't have an idea if you can even do that.
    fn new(owner: String, balance: f64) -> Self {
        Account { 
            owner, 
            balance, 
        }
    }

    //Herec"self" is lower case because we're referring to the value inside an instance of the struct but not the blueprint of the struct itself.
    //Here we reference the already struct instance using "&mut" because we don't want to take ownership of it since, we're just editing the value within it. 
    //Atleast that's what i understand.
    fn deposit(&mut self, amount: f64) {
        self.balance += amount
    }
}

//We take reference of the generic type "Option" with the type parameter "struct Account" as an argument because we want to change the instance stored in the "account variable"
//back in main.
fn deposit(account: &mut Option<Account>) -> Result<String, String> {
    //Here we call Some() because we need to modify the "account" variable back in main but its a generic enum type. We can't do "unwrapped_var.deposit" because that's simply not a feature with that type.
    //However the variable inside it is a struct type so, all we have to do is take it out of the generic enum then we can add or change the values inside the struct.
    if let Some(unwrapped_var) = account {
        let amount = get_f64("enter deposit amount: ");
        unwrapped_var.deposit(amount);
        Ok(format!("new balance: {}", unwrapped_var.balance))
    } else {
        Err(String::from("Account does not exist yet!"))
    }
}

//"Result<T, E>" is similar to the Option enum except, it checks if the value is true or false. Something like a?1:0. You can also return custom strings in both Ok and Err cases.

fn withdraw(account: &mut Option<Account>) -> Result<String, String> {
    if let Some(var) = account {
        let user = get_input_string("enter username: ");
        if user == var.owner {
            let withdraw_amount = get_f64("enter withdraw amount: ");
            if withdraw_amount > var.balance {
                Err(format!("insufficient funds: {}", var.balance))
            } else {
                var.balance -= withdraw_amount;
                Ok(format!("new balance: {}", var.balance))
            }
        } else {
            Err(format!("user {} does not exist", user))
        }
    } else {
        Err(String::from("No accounts exist yet."))
    }
}