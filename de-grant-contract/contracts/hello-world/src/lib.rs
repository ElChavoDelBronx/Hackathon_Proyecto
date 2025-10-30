#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, token, Address, Env, Symbol, Map, vec, String, Vec};

// Claves de almacenamiento
const ADMIN: Symbol = symbol_short!("ADMIN");
const TOKEN: Symbol = symbol_short!("TOKEN");
const ELIGIBLE: Symbol = symbol_short!("ELIGIBLE");

#[contract]
pub struct Contract;

// This is a sample contract. Replace this placeholder with your own contract logic.
// A corresponding test example is available in `test.rs`.
//
// For comprehensive examples, visit <https://github.com/stellar/soroban-examples>.
// The repository includes use cases for the Stellar ecosystem, such as data storage on
// the blockchain, token swaps, liquidity pools, and more.
//
// Refer to the official documentation:
// <https://developers.stellar.org/docs/build/smart-contracts/overview>.
#[contractimpl]
impl Contract {
    /// FUNCIÓN DE INICIALIZACIÓN
    /// Establece el admin y el token de recompensa.
    pub fn initialize(env: Env, admin: Address, token_id: Address) {
        if env.storage().instance().has(&ADMIN) {
            panic!("Ya inicializado");
        }
        env.storage().instance().set(&ADMIN, &admin);
        env.storage().instance().set(&TOKEN, &token_id);
        // Inicializa el mapa de usuarios elegibles como vacío
        env.storage().persistent().set(&ELIGIBLE, &Map::<Address, i128>::new(&env));
    }

    /// FUNCIÓN DE FINANCIACIÓN
    /// El admin también puede simplemente enviar tokens a la 
    /// dirección de este contrato usando una wallet.
    /// Esta función es solo una forma explícita de hacerlo.
    pub fn fund(env: Env, funder: Address, amount: i128) {
        funder.require_auth(); // Quien fondea debe autorizar
        let token_id: Address = env.storage().instance().get(&TOKEN).unwrap();
        let token_client = token::Client::new(&env, &token_id);

        // Transfiere fondos DESDE el 'funder' HACIA este contrato
        token_client.transfer(
            &funder, 
            &env.current_contract_address(), 
            &amount
        );
    }

    /// FUNCIÓN DE AUTORIZACIÓN (Solo Admin)
    /// El admin llama a esto para hacer elegible a un usuario.
    pub fn set_eligible(env: Env, user: Address, amount: i128) {
        // Solo el admin puede autorizar recompensas
        let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        admin.require_auth();

        // Carga el mapa actual de usuarios elegibles
        // Usamos almacenamiento 'persistent' para estructuras como Map
        let mut eligible_map: Map<Address, i128> = env.storage()
            .persistent()
            .get(&ELIGIBLE)
            .unwrap();

        // Agrega o actualiza la cantidad hacia el usuario
        eligible_map.set(user, amount);

        // Guarda el dato actualizado en la estructura map
        env.storage().persistent().set(&ELIGIBLE, &eligible_map);
    }

    /// FUNCIÓN DE RECLAMO (Abierta a todos)
    /// El usuario elegido podrá llamar a esta función para retirar su beca.
    pub fn claim_schoolarship(env: Env, user: Address) {
        //El usuario que reclama debe firmar la transacción, autenticando que es él.
        user.require_auth();

        //Carga el mapa de usuarios elegibles
        let mut eligible_map: Map<Address, i128> = env.storage()
            .persistent()
            .get(&ELIGIBLE)
            .unwrap();

        //Busca la recompensa del usuario
        // .get() devuelve un Some(monto) o None
        if let Some(reward_amount) = eligible_map.get(user.clone()) {
            
            // Elimina al usuario del mapa para que no pueda reclamar
            // para que no pueda reclamar dos veces tan pronto.
            eligible_map.remove(user.clone());
            
            // Guarda el mapa (ya sin el usuario anteriormente elegido)
            env.storage().persistent().set(&ELIGIBLE, &eligible_map);

            // Prepara la transferencia del token
            let token_id: Address = env.storage().instance().get(&TOKEN).unwrap();
            let token_client = token::Client::new(&env, &token_id);

            // Paga la beca
            // Transfiere DESDE este contrato HACIA el 'user'
            token_client.transfer(
                &env.current_contract_address(),  //desde
                &user,                            //hacia
                &reward_amount
            );

        } else {
            // Si el usuario no está en el mapa, falla la transacción.
            panic!("No eres elegible o ya reclamaste tu recompensa");
        }
    }
    pub fn hello(env: Env, to: String) -> Vec<String> {
        vec![&env, String::from_str(&env, "Hello"), to]
    }
}

mod test;
