// Esta línea le dice a Rust que solo compile este código 
// cuando ejecutes el comando 'cargo test'
#[cfg(test)] 

    use crate::{SchoolarshipsContract, SchoolarshipsContractClient};

    // Importa todo el código de tu contrato principal
    use super::*; 

    // Importa las utilidades de prueba de Soroban
    use soroban_sdk::{Env, Address, testutils::Address as _};

    // 'Macro' que identifica esto como una prueba individual
    #[test] 
    fn test_initialization() {
        // 1. SETUP: Crear el entorno falso
        let env = Env::default();
        
        // 2. REGISTRAR: Desplegar el contrato en el entorno
        let contract_id = env.register_contract(None, SchoolarshipsContract);
        let client = SchoolarshipsContractClient::new(&env, &contract_id);

        // 3. CREAR CUENTAS FALSAS
        let admin = Address::random(&env);
        let token_id = Address::random(&env);

        // 4. INVOCAR: Llamar a la función 'initialize'
        client.initialize(&admin, &token_id);

        // 5. VERIFICAR: Usar 'assert_eq!' para comprobar el resultado
        // ¿Se guardó el admin correctamente en el almacenamiento?
        let stored_admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        assert_eq!(stored_admin, admin);
        
        // ¿Se guardó el token correctamente?
        let stored_token: Address = env.storage().instance().get(&TOKEN).unwrap();
        assert_eq!(stored_token, token_id);
    }

    #[test]
    #[should_panic(expected = "No eres elegible")] // Esta prueba ESPERA que falle
    fn test_claim_without_being_eligible() {
        // 1. SETUP... (inicializar el contrato igual que arriba)
        let env = Env::default();
        let contract_id = env.register_contract(None, SchoolarshipsContract);
        let client = SchoolarshipsContractClient::new(&env, &contract_id);
        let admin = Address::random(&env);
        let token_id = Address::random(&env);
        client.initialize(&admin, &token_id);

        // 2. CREAR UN USUARIO
        let user = Address::random(&env);

        // 3. INVOCAR (El momento de la verdad)
        // El admin nunca llamó a 'set_eligible' para este usuario
        // Esta llamada DEBE fallar
        client.claim_schoolarship(&user);
        
        // 4. VERIFICAR: La macro '#[should_panic]' confirma que la 
        // transacción falló (entró en 'panic') como queríamos.
    }
