use crate::helper_functions::{get_f64, get_i32, get_input_string};

mod helper_functions;

fn main() {
    //Declares the account variable early without any value.
    let mut account:Option<Account> = None;

   loop {
     let input: i32 = get_i32("new account [1] deposit[2] withdraw[3] exit[0]: ");
     match input {
        0 => return,
            //Some() returns the data that was called back to the account variable outside of match transferring ownership back to it.
        1 => account = Some(create_account()),
        2 => deposit(&mut account),
        3 => withdraw(&mut account),
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
fn deposit(account: &mut Option<Account>) {
    //Here we call Some() because we need to modify the "account" variable back in main but its a generic enum type. We can't do "unwrapped_var.deposit" because that's simply not a feature with that type.
    //However the variable inside it is a struct type so, all we have to do is take it out of the generic enum then we can add or change the values inside the struct.
    if let Some(unwrapped_var) = account {
        let amount = get_f64("enter deposit amount: ");
        unwrapped_var.deposit(amount);
        println!("new balance: {}", unwrapped_var.balance);
    } else {
        println!("Account does not exist yet!");
    }
}

fn withdraw(account: &mut Option<Account>) {
    if let Some(unwrapped_var) = account {
        let new_user = get_input_string("enter username: ");
        if new_user == unwrapped_var.owner {
            let withdraw_amount = get_f64("enter withdraw amount: ");
            if withdraw_amount > unwrapped_var.balance {
                println!("insuffucient funds: {}", unwrapped_var.balance)
            } else {
                unwrapped_var.balance -= withdraw_amount;
                println!("New balance: {}", unwrapped_var.balance)
            }
        } else {
            println!("user: {} does not exist.", new_user)
        }
    }
}