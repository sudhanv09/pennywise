use dioxus::prelude::*;

mod components;
mod db;
mod models;
mod repository;
mod screens;

use components::app_dock::AppDock;
use screens::{
    home::Home,
    settings::Settings,
    accounts::Accounts,
    categories::Categories,
    goals::Goals,
    loans::Loans,
    transaction_form::{AddTransaction, EditTransaction},
    transactions::Transactions,
};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[derive(Clone, Routable, PartialEq, Debug)]
enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        Home,
        #[route("/settings")]
        Settings,
        #[route("/transactions")]
        Transactions,
    #[end_layout]
    #[route("/transaction/new")]
    AddTransaction,
    #[route("/transaction/:id/edit")]
    EditTransaction { id: i32 },
    #[route("/settings/accounts")]
    Accounts,
    #[route("/settings/categories")]
    Categories,
    #[route("/settings/goals")]
    Goals,
    #[route("/settings/loans")]
    Loans,
}

#[component]
fn AppLayout() -> Element {
    rsx! {
        Outlet::<Route> {}
        AppDock {}
    }
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| {
        db::init().expect("in-memory db failed")
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}
