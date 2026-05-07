use crate::{
    blockchain::{header::Header, block_body::BlockBody}, 
    state_machine::StateMachine,
    blockchain::hash::HashStruct,
    blockchain::transaction::Transaction
}; 

pub enum ActionType {
    
}

pub struct Action {

}

pub enum BlockState {
    Rejected, 
    Processing, 
    Completed 
}

pub struct Block {
    header: Header, 
    body: BlockBody
}

impl Block {
    fn genesis() -> Self {
        let parent = HashStruct::default();
        let first_transaction = Transaction::default();

        // TODO: Make sure the state_root is the appropriate HashStruct rather than it's current placeholderqw
        Block {
            header: Header::new(parent, first_transaction.get_hash(), HashStruct::default()), 
            body: BlockBody::new(vec!(first_transaction))
        }
    }
}

impl StateMachine for Block {
    type State = BlockState; 
    type Transition = Action; 

    fn switch_state_next(starting_state: &BlockState, t: &Action) -> BlockState {
        todo!("Implement ability to check whether attempted action isv valid. An Action should provide the details 
                1) What type of action is being done
                2) What quantity
                3) By who (This one is a uid)
            "); 
        
        todo!("Change the state into a processing state. This allows me to do the work required and lock the list");
        todo!("Change it back into a completed processing state")
    }

    fn human_readable() -> String {
        "BlockChain State machine".to_string()
    }
}