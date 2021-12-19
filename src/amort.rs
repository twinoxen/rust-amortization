use chrono::{DateTime, Duration, Utc};

#[derive(Debug, PartialEq)]
pub struct AmortizedDatum {
    payment_number: i32,
    payment_date: DateTime<Utc>,
    payment: f32,
    principle: f32,
    interest: f32,
    total_interest: f32,
    balance: f32,
}

pub fn amortize(
    loan_amount: f32,
    terms_in_months: i16,
    annual_interest_rate: f32,
) -> Vec<AmortizedDatum> {
    let mut amortized: Vec<AmortizedDatum> = Vec::new();

    let interest_to_decimal = annual_interest_rate / 100 as f32;
    let monthly_interest_rate = interest_to_decimal / 12 as f32;
    let monthly_interest_rate_by_term =
        f32::powi(1.0 + monthly_interest_rate, terms_in_months as i32);
    let total_monthly_payment =
        (loan_amount * monthly_interest_rate * monthly_interest_rate_by_term)
            / (monthly_interest_rate_by_term - 1.0);
    let rounded_monthly_payment = round(total_monthly_payment);

    let mut payment_date = Utc::now();
    let mut balance = loan_amount;
    let mut total_interest: f32 = 0.0;

    let mut count = 1;
    while balance > 0.0 {
        let payment = rounded_monthly_payment;
        let interest = monthly_interest_rate * balance;

        let mut principle = payment - interest;

        if principle > balance {
            principle = balance
        }

        total_interest += interest;
        balance -= principle;

        amortized.push(AmortizedDatum {
            payment_number: count,
            payment_date: payment_date,
            payment: round(payment),
            principle: round(principle),
            interest: round(interest),
            total_interest: round(total_interest),
            balance: round(balance),
        });

        payment_date = payment_date + Duration::days(30); // TODO: make this the first of every month
        count += 1;
    }

    amortized
}

fn round(amount: f32) -> f32 {
    (amount * 100.0).ceil() / 100.0
}

#[cfg(test)]
mod tests {
    use crate::amort::*;

    #[test]
    fn first_amortized_entry_is_correct() {
        let amortized_table: Vec<AmortizedDatum> = amortize(300000.0, 360, 5.0);
        let first_entry: &AmortizedDatum = &amortized_table[0];

        assert_eq!(first_entry.payment, 1610.46);
        assert_eq!(first_entry.principle, 360.46);
        assert_eq!(first_entry.interest, 1250.01);
        assert_eq!(first_entry.total_interest, 1250.01);
        assert_eq!(first_entry.balance, 299639.53);
    }

    #[test]
    fn last_amortized_entry_is_correct() {
        let amortized_table: Vec<AmortizedDatum> = amortize(300000.0, 360, 5.0);
        let last_entry: &AmortizedDatum = &amortized_table[amortized_table.len() - 1];

        assert_eq!(last_entry.payment, 1610.46);
        assert_eq!(last_entry.principle, 3.81);
        assert_eq!(last_entry.interest, 0.02);
        assert_eq!(last_entry.total_interest, 279769.75);
        assert_eq!(last_entry.balance, 0.0);
    }
}
