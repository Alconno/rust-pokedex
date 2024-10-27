use async_trait::async_trait;
use error::Error;
use support::store::models::user_pokedex::UserPokedex;

use super::{contract::{AdminPokedexesContract, PgRepositoryContract, PgServiceContract}, data::PokedexAttributes};

pub struct AdminPokedexes<A: PgRepositoryContract, B: PgServiceContract> {
    pub repository: A,
    pub service: B,
}

#[async_trait]
impl<A, B> AdminPokedexesContract for AdminPokedexes<A, B>
where
    A: PgRepositoryContract + Sync + Send,
    B: PgServiceContract + Sync + Send,
{
    /// Paginate pokedexes.
    async fn paginated(&self, pokedex_attributes: PokedexAttributes) -> Result<Vec<UserPokedex>, Error>{
        // Get paginated pokedexes from repository
        let pokedexes = self.repository.paginated(&pokedex_attributes).await?;

        Ok(pokedexes)
    }

    /// Delete user pokedex by ID.
    async fn delete(&self, user_pokedex_id: &str) -> Result<(), Error>{
        let user_pokedex_data = self.repository.get_by_id(&user_pokedex_id).await?;
        let user_attempt = self.repository.get_user_attempt_by_user_id_and_pokemon_id(&user_pokedex_data.user_id, &user_pokedex_data.pokemon_id).await?;

        // Delete user pokedex and if deletion is successful, set attempt is_success to false so pokemon can be caught again
        self.service.delete(&user_pokedex_id).await?;
        self.service.set_attempt_success_false(&user_attempt.id).await?;

        Ok(())
    }
}
