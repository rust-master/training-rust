enum Payment {
    Cash(f32),
    CreditCard(String, f32),
    DebitCard(DebitData),
    Crypto { account_id: String, amount: f32 },
}

struct DebitData {
    pub card_number: String,
    pub amount: f32,
}

fn main() {
    let some_payment = Payment::Cash(100.);
    process_payment(some_payment);

    let cc_payment = Payment::CreditCard("CC_Num".to_string(), 250.);
    process_payment(cc_payment);

    let debit_payment = Payment::DebitCard(DebitData {
        card_number: "Debit Num".to_string(),
        amount: 400.,
    });
    process_payment(debit_payment);

    let crypto_payment = Payment::Crypto {
        account_id: "abc 123".to_string(),
        amount: 1000.,
    };
    process_payment(crypto_payment);
}

fn process_payment(some_payment: Payment) {
    match some_payment {
        Payment::Cash(amt) => {
            println!("Paying with cash... {}", amt);
        }
        Payment::CreditCard(some_string, some_f32) => {
            println!(
                "Paying with CreditCard... some_string {}, some_f32 {}",
                some_string, some_f32
            );
        }
        Payment::DebitCard(data) => {
            println!(
                "Paying with debit.... card_number {} , amount {}",
                data.card_number, data.amount
            );
        }
        Payment::Crypto { account_id, amount } => {
            println!(
                "Paying with crypto... account_id {}, amount {}",
                account_id, amount
            );
        }
    };
}
