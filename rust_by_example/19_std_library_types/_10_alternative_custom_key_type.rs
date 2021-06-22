use std::collections::HashMap;

// Eq requires that you derive PartialEq on the type.
#[derive(PartialEq, Eq, Hash)]
struct Account<'a> {
	username: &'a str,
	password: &'a str,
}

struct AccountInfo<'a> {
	name: &'a str,
	email: &'a str,
}

type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

fn try_logon<'a>(accounts: &Accounts<'a>, user: &'a str, pass: &'a str) {
	println!("username: {}", user);
	println!("password: {}", pass);
	println!("Attempting logon...");

	let logon = Account {
		username: user,
		password: pass,
	};

	match accounts.get(&logon) {
		Some(account_info) => {
			println!("Successful logon!");
			println!("Name: {}", account_info.name);
			println!("Email: {}", account_info.email);
		}, 
		_ => println!("Login failed!"),
	}
}

fn main() {
	let mut accounts: Accounts = HashMap::new();

	let account = Account {
		username: "j.everyman",
		password: "password123",
	};
	let account_info = AccountInfo {
		name: "John Everyman",
        email: "j.everyman@email.com",
	};

	accounts.insert(account, account_info);

	try_logon(&accounts, "j.everyman", "psasword123");

	println!("==========================");

	try_logon(&accounts, "j.everyman", "password123");
}