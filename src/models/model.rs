use chrono::{NaiveDate, NaiveTime};


#[derive(Debug, Clone)]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub starting_balance: f32,
    pub icon: String,
    pub currency: String,
}

#[derive(Debug, Clone)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub icon: String,
}

#[derive(Debug, Clone)]
pub struct Goals {
    pub id: i32,
    pub name: String,
    pub target: f32,
    pub current: f32,
    pub deadline: Option<f32>,
}

#[derive(Debug, Clone)]
pub struct Loans {
    pub id: i32,
    pub name: String,
    pub total_amount: f32,
    pub paid_amount: f32,
    pub due: f32,
    pub is_lender: bool
}

#[derive(Debug, Clone)]
pub enum BillingCycle {
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionType {
    Income,
    Expense,
    Transfer,
}

#[derive(Debug, Clone)]
pub struct Subscription {
    pub id: i32,
    pub name: String,
    pub billing_cycle: BillingCycle,
    pub next_billing: f32,
}

#[derive(Debug, Clone)]
pub struct Transactions {
    pub id: i32,
    pub title: String,
    pub amount: f32,
    pub tx_date: NaiveDate,
    pub tx_time: NaiveTime,
    pub tx_type: TransactionType,
    pub category: i16,
    pub account: i16,
    pub description: String,
}