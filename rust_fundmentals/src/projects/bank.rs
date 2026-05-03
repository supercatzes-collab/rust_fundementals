
use std::collections::HashMap;

use crate::helper_functions::*;

//Make a hashmap called "accounts" with values "i32" and "struct".
//Call function that returns "k: i32" and "v: struct instance".
//Insert into hashmap using hashmap.insert(k: i32, v: struct instance).
pub fn bank() {
    let mut accounts: HashMap<i32, Account> = HashMap::new();

    loop {
        let input = get_i32("exit[0] new_account[1] deposit[2] withdraw[3] list account[4] transfer[5]: ");

        match input {
            0 => return,
            1 => {let new_user = new_account();
                    accounts.insert(new_user.id, new_user);
                 },
            2 => match deposit(&mut accounts) {
                    Ok(msg) => println!("{}", msg),
                    Err(e) => println!("{}", e),
                 },
            3 => match withdraw(&mut accounts) {    
                    Ok(msg) => println!("{}", msg),
                    Err(e) => println!("{}", e),
                 },
            4 => for (id, accounts) in &accounts {
                    println!("{} | {} | {}", id, accounts.user, accounts.balance)
                 },
            5 => match transfer(&mut accounts){
                    Ok(msg) => println!("{}", msg),
                    Err(e) => println!("{}", e),
                 },    
            _ => println!("unknown option"),
        }
    }
}

struct Account {
    id: i32,
    user: String,
    balance: f64,
}
//impl is a way to add functionally to types. In this case its a function that automatically creates a new instance of struct "Account" then returning it to whoever calls it.
//Notice how "Self" is capitalized? This is because the function "new()" is referring to the type itself. However if we're referring to the contents inside the struct then we'd use "self"
impl Account {
    fn new(id: i32, user: String, balance: f64) -> Self {
        Account {id, user, balance}
    }
}

fn new_account() -> Account {
    let user = get_input_string("please enter username: ");
    let id = get_i32("please input PIN: ");
    let balance = get_f64("deposit amount: ");

    //"::" Means "look inside x then find y".
    Account::new(id, user, balance)
}

//Here our fn take reference of hashmap and also specify the types within the hashmap that we want to use. In this case its an i32 and a struct type.
//We use a match case instead of if statements because it allows us to handle mutiple conditions in one codeblock instead of having multiple nested if statements. 
//We use a match case here because it allows us to handle multiple conditions without having to resort t having nested if else statements.
//In-order to actually use match to compare input and the struct instance inside the hashmap, we use the ".get_mut" method.
//The ".get_mut" returns the struct instance we want but its converted into an "Option<&mut Account>" so we can use Some() and None() variants. ".get_mut" Also requires the argument to be a reference regardless if its unnecessary and it also always returns an "Option<T>" type.
//Then finally we exploit the "Option<T>" feature by using the variant Some() instead of if statements.
//Then we use the enum type "Result<ok,e>" return a flag. Then have the caller decide what to do with it.
fn deposit(accounts: &mut HashMap<i32, Account>) -> Result<String, String> {
    let input = get_i32("please input your pin: ");

    match accounts.get_mut(&input) { 
        Some(account) => {
            let input = get_f64("input deposit amount: ");
            account.balance += input;
            Ok(format!("new balance: {}", account.balance))
        },
        None => Err(format!("account does not exist: {}.", input))
    } 
}

fn withdraw(accounts: &mut HashMap<i32, Account>) -> Result<String, String> {
    let input = get_i32("please input PIN: ");

    match accounts.get_mut(&input) {
        Some(account) =>  {
            let withdrawal = get_f64("please input withdrawal amount: ");
            if withdrawal > account.balance {
                Err(format!("insufficient funds: {}", account.balance))
            } else {
                account.balance -= withdrawal;
                Ok(format!("new balance: {}", account.balance))
            }
        },
        None => Err(format!("account does not exist {}", input))
    }
}

fn transfer(accounts: &mut HashMap<i32, Account>) -> Result<String, String> {
    let from_account = get_i32("input PIN: ");
    let to_account = get_i32("input PIN to deposit to: ");

    if !accounts.contains_key(&from_account) {
        return Err(format!("account: {} does not exist", from_account));
    }

    if !accounts.contains_key(&to_account) {
        return Err(format!("account: {} does not exist", to_account));
    }

    if let Some(mut from_id) = accounts.remove(&from_account) { //We remove the instance of the first account because we can only have one mut reference per value, so to work around this, we can just remove it from the hashmap temporarily then insert it back in.
        let amount = get_f64("enter transfer amount: "); 
        if from_id.balance < amount {
            accounts.insert(from_account, from_id);                 //Insert the id of 1st account back to the hashmap.
            return Err(format!("insufficient funds: {}", amount))
        }
        from_id.balance -= amount;
        accounts.get_mut(&to_account).unwrap().balance += amount;        //Get the struct instance of the 2nd account the unwrapt it from the Option() to add our amount to the balance.
        accounts.insert(from_account, from_id);                     //Insert the id of the 1st account back to the hashmap again.
    }
    Ok(String::from("success"))
}
