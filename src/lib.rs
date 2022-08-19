use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
// use near_sdk::collections::Vector;
use near_sdk::{env, near_bindgen, AccountId};
// courier service
//
/*  packages
*      -> id
*      -> source
*      -> destination
*      -> from whom
*      -> to whom
*      -> date_sent
*      -> date_recieved
*      -> weight
*      -> is fragile
*
*   Station
*      -> name
*      -> list/ collection == vectors  <packages>
*
// *   -> add package
// *   -> pacage collected
*/

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Packages {
    id: u8,
    source: String,
    destination: String,
    from_whom: String,
    to_whom: String,
    date_sent: String,
    date_recieved: String,
    weight: u8,
    is_fragile: bool,
}

// stations
//   -> muliple packages
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Station {
    name: String,
    location: String,
}

// impl Default for Station {
//     fn default() -> Self {
//         Station {
//             name: "".to_string(),
//             packages: vec![], //Vector::new(b"r".to_vec()),
//         }
//     }
// }

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct CourierService {
    station: Vec<Station>,
    packages: Vec<Packages>,
}

#[near_bindgen]
impl CourierService {
    #[init]
    #[private]
    pub fn new() -> Self {
        let mut data: Vec<Station> = vec![];
        let  package: Vec<Packages> = vec![];

        data.push(Station {
            name: "kisumu_main".to_string(),
            location: "kisumu".to_string(),
        });
        data.push(Station {
            name: "kisumu_kogello".to_string(),
            location: "kisumu".to_string(),
        });
        Self {
            station: data,
            packages: package,
        }
    }
    /**
     * near call guest-book.testnet add_package '{"source": "nairobu","destination": "kisumu" , ...}' --account-id example-acct.testnet
     */
    pub fn add_package(
        &mut self,
        source: String,
        destination: String,
        from: String,
        to_whom: String,
        date_sent: String,
        date_received: String,
        weight: u8,
        is_fragile: bool,
    ) -> (String, Option<u8>) {
        let mut source_exist = false;
        let mut destination_exist = false;

        for elem in self.station.iter() {
            if elem.name == source {
                source_exist = true;
            }
            if elem.name == destination {
                destination_exist = true;
            }
        }

        if source_exist == false {
            return (String::from("source does not exist"), None);
        }
        if destination_exist == false {
            return (String::from("destination does not exist"),None);
        }
        if source == destination {
            return (String::from(" source cannot be equal to destination"), None);
        }

        let random_number = match env::random_seed().get(0) {
            Some(x) => *x,
            None => 0,
        };
        let package = Packages {
            id: random_number,
            source: source,
            destination: destination,
            from_whom: from,
            to_whom: to_whom,
            date_sent: date_sent,
            date_recieved: date_received,
            weight: weight,
            is_fragile: is_fragile,
        };

        self.packages.push(package);

        return (String::from("okay"), Some(random_number));
    }

    pub fn collect_package(&mut self,  package_id: u8) {
        for (index, elem) in self.packages.iter_mut().enumerate() {
            if elem.id == package_id {
                
                self.packages.remove(index);

                break;
            }
        }
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE


    #[test]
    fn add_package() {
        let mut courier = CourierService::new();
        courier.add_package("kisumu_main".to_string(),
         "kisumu_kogello".to_string(),
        "richard".to_string(), "moris".to_string(), "12/10/2022".to_string(), "12/10/2022".to_string(), 22,false);
        assert_eq!(courier.packages.len(), 1);
    }


    #[test]
    fn collect_package() {
        let mut courier = CourierService::new();
      let res=   courier.add_package("kisumu_main".to_string(),
         "kisumu_kogello".to_string(),
        "richard".to_string(), "moris".to_string(), "12/10/2022".to_string(), "12/10/2022".to_string(), 22,false);

        
       match res.1{
        Some(k)=>{
            courier.collect_package(k);
            assert_eq!(courier.packages.len(),0)
        }
        None=>{
            panic!("failed to get package")
        }
       }
    }

}
