use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, Vector};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Promise};
 
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
 
#[warn(dead_code)]
fn _one_near() -> u128 {
    u128::from_str_radix("1000000000000000000000000", 10).unwrap()
}
 
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Packages {
    id: u64,
    source: String,
    destination: String,
    from_whom: AccountId,
    to_whom: AccountId,
    cost: u128,
    date_recieved: u64,
    date_sent: u64,
    weight: u8,
    is_fragile: bool,
    is_received: bool,
}
 
// stations
//   -> muliple stations can be in a town
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Station {
    name: String,
    id: u64,
}
 
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct CourierService {
    station: UnorderedMap<String, Vector<Station>>,
    packages: Vector<Packages>,
}
 
impl Default for CourierService {
    fn default() -> Self {
        let mut station: UnorderedMap<String, Vector<Station>> = UnorderedMap::new(b"p".to_vec());
 
        let mut nairobi_stations: Vector<Station> = Vector::new(b"r".to_vec());
        let mut kisumu_stations: Vector<Station> = Vector::new(b"r".to_vec());
 
        nairobi_stations.push(&Station {
            name: "city-center".to_string(),
            id: 123,
        });
        nairobi_stations.push(&Station {
            name: "ngara".to_string(),
            id: 123,
        });
        kisumu_stations.push(&Station {
            name: "kondele".to_string(),
            id: 123,
        });
        kisumu_stations.push(&Station {
            name: "posta".to_string(),
            id: 123,
        });
        station.insert(&"kisumu".to_string(), &nairobi_stations);
        station.insert(&"nairobi".to_string(), &kisumu_stations);
 
        CourierService {
            packages: Vector::new(b"r".to_vec()),
            station: station,
        }
    }
}
 
#[near_bindgen]
impl CourierService {
    pub fn get_stations(&self) -> Vec<Station> {
        let mut data: Vec<Station> = vec![];
 
        for elem in self.station.values_as_vector().iter() {
            for st in elem.iter() {
                data.push(st);
            }
        }
 
        data
    }
 
    pub fn get_packages(&self) -> Vec<Packages> {
        self.packages.to_vec()
    }
 
    pub fn register_station(&mut self, town: String, name: String) {
        let  stations_in_town = self.station.get(&town);
        let station = &Station {
            name: name,
            id: env::block_timestamp(),
        };
        match stations_in_town {
            Some(mut k) => k.push(station),
            None => {
                let mut data: Vector<Station> = Vector::new(b"r".to_vec());
                data.push(station);
                self.station.insert(&town, &data);
            }
        }
    }
 
    #[payable]
    pub fn add_package(
        &mut self,
        source: String,
        destination: String,
        cost: u128,
        to_whom: String,
        weight: u8,
        is_fragile: bool,
    ) -> u64 {
        let pkg_id = env::block_timestamp();
        if env::account_balance() < cost {
            env::panic_str("Not enough balance to pay for package");
        }
        let pkg = Packages {
            id: pkg_id,
            source: source,
            destination: destination,
            from_whom: env::signer_account_id(),
            to_whom: AccountId::new_unchecked(to_whom),
            cost: cost,
            date_sent: env::block_timestamp(),
            date_recieved: env::block_timestamp(),
            weight: weight,
            is_fragile: is_fragile,
            is_received: false,
        };
        self.packages.push(&pkg);
 
        Promise::new(env::signer_account_id()).transfer(cost as u128);
 
        pkg_id
    }
 
    pub fn collect_package(&mut self, package_id: u64) -> String {
        let mut pkg_index: Option<u64> = None;
        for (index, elem) in self.packages.iter().enumerate() {
            if elem.id == package_id {
                pkg_index = Some(index as u64);
 
                break;
            }
        }
 
        match pkg_index {
            Some(k) => {
                let mut pkg = self.packages.get(k).unwrap();
                if pkg.to_whom == env::signer_account_id() {
                    pkg.is_received= true;
 
                    self.packages.replace(k, &pkg);
                    "okay".to_string()
                } else {
                    env::log_str("only recepient can pick up package");
                    "error".to_string()
                }
            }
            None => {
                env::log_str("package not found");
                "errors".to_owned()
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
    use near_sdk::test_utils::{ VMContextBuilder};
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
    fn register_station() {
        let mut courier = CourierService::default();
        courier.register_station("nakuru".to_string(), "railways".to_string());
 
        assert_eq!(courier.station.len(), 3)
    }
 
    #[test]
    fn add_package() {
        let user = AccountId::new_unchecked("kenn.testnet".to_string());
        let mut _context = get_context(user.clone());
        let bal = _one_near() * 20;
        _context.attached_deposit(bal);
        _context.account_balance(bal);
        testing_env!(_context.build());
 
        let mut courier = CourierService::default();
        courier.add_package(
            "kisumu_main".to_string(),
            "kisumu_kogello".to_string(),
            _one_near(),
            "moris.testnet".to_string(),
            22,
            false,
        );
        assert_eq!(courier.packages.len(), 1);
    }
 
    #[test]
    fn collect_package() {
        let user = AccountId::new_unchecked("kenn.testnet".to_string());
        let mut _context = get_context(user.clone());
        let bal = _one_near() * 20;
        _context.attached_deposit(bal);
        _context.account_balance(bal);
        testing_env!(_context.build());
 
        let mut courier = CourierService::default();
        let package_id = courier.add_package(
            "kisumu_main".to_string(),
            "kisumu_kogello".to_string(),
            _one_near(),
            "moris.testnet".to_string(),
            22,
            false,
        );
        assert_eq!(courier.packages.len(), 1);
 
        let user2 = AccountId::new_unchecked("moris.testnet".to_string());
        _context.signer_account_id(user2);
        testing_env!(_context.build());
        let res= courier.collect_package(package_id);
 
        assert_eq!(res,"okay".to_owned());
        let mut pkg: Option<Packages> = None;
 
        for elem in courier.packages.iter() {
            if elem.id == package_id {
                pkg = Some(elem);
            }
        }
 
        match pkg {
            Some(k) => {
                assert_eq!(k.is_received, true);
            }
            None => env::panic_str("package not fund"),
        }
    }
}
