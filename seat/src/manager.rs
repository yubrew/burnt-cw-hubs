pub mod contract_manager {
    use cosmwasm_std::{Empty, StdError, Uint64};
    use cw_storage_plus::{Item, Map};
    use metadata::Metadata;
    use ownable::Ownable;
    use redeemable::Redeemable;
    use sellable::Sellable;
    use std::{cell::RefCell, rc::Rc};
    use thiserror::Error;

    use burnt_glue::manager::Manager;
    use token::Tokens;

    use crate::state::{SeatMetadata, TokenMetadata};

    #[derive(Error, Debug)]
    pub enum ManagerError {
        #[error("{0}")]
        Std(#[from] StdError),

        #[error("Custom Error val: {val:?}")]
        CustomError { val: String },
        // Add any other custom errors you like here.
        // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
    }

    pub fn get_manager() -> Manager {
        let mut contract_manager = Manager::new();
        // register all modules required for call
        let owner: Rc<RefCell<Ownable>> = Rc::new(RefCell::new(Ownable::default()));
        contract_manager
            .register("ownable".to_string(), owner.clone())
            .unwrap();

        let metadata = Rc::new(RefCell::new(Metadata::new(
            Item::<SeatMetadata>::new("metadata"),
            owner.clone(),
        )));
        contract_manager
            .register("metadata".to_string(), metadata)
            .unwrap();

        let seat_token = Rc::new(RefCell::new(
            Tokens::<TokenMetadata, Empty, Empty, Empty>::new(
                cw721_base::Cw721Contract::default(),
                Some("uturnt".to_string()),
            ),
        ));
        contract_manager
            .register("seat_token".to_string(), seat_token.clone())
            .unwrap();

        let redeemable = Rc::new(RefCell::new(Redeemable::new(Item::new("redeemed_items"))));
        contract_manager
            .register("redeemable".to_string(), redeemable.clone())
            .unwrap();

        let sellable_token = Rc::new(RefCell::new(
            Sellable::<TokenMetadata, Empty, Empty, Empty>::new(
                seat_token,
                owner,
                Map::<&str, Uint64>::new("listed_tokens"),
            ),
        ));
        contract_manager
            .register("sellable_token".to_string(), sellable_token)
            .unwrap();

        contract_manager
    }
}
